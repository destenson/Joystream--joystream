// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

#[cfg(test)]
mod tests;

mod class;
mod entity;
mod errors;
mod helpers;
mod mock;
mod operations;
mod permissions;
mod schema;

pub use class::*;
pub use entity::*;
pub use errors::*;
pub use helpers::*;
pub use operations::*;
pub use permissions::*;
pub use schema::*;

use core::fmt::Debug;
use core::hash::Hash;
use core::ops::AddAssign;

use codec::{Codec, Decode, Encode};
use rstd::collections::{btree_map::BTreeMap, btree_set::BTreeSet};
use rstd::prelude::*;
use runtime_primitives::traits::{MaybeSerializeDeserialize, Member, One, SimpleArithmetic, Zero};
use srml_support::{
    decl_event, decl_module, decl_storage, dispatch, ensure, traits::Get, Parameter,
    StorageDoubleMap,
};
use system::ensure_signed;

#[cfg(feature = "std")]
pub use serde::{Deserialize, Serialize};

use core::debug_assert;

/// Type, used in diffrent numeric constraints representations
type MaxNumber = u32;

pub trait Trait: system::Trait + ActorAuthenticator + Debug + Clone {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    /// Nonce type is used to avoid data race update conditions, when performing property value vector operations
    type Nonce: Parameter
        + Member
        + SimpleArithmetic
        + Codec
        + Default
        + Copy
        + Clone
        + One
        + Zero
        + MaybeSerializeDeserialize
        + Eq
        + PartialEq
        + Ord
        + From<u32>;

    /// Type of identifier for classes
    type ClassId: Parameter
        + Member
        + SimpleArithmetic
        + Codec
        + Default
        + Copy
        + Clone
        + One
        + Hash
        + Zero
        + MaybeSerializeDeserialize
        + Eq
        + PartialEq
        + Ord;

    /// Type of identifier for entities
    type EntityId: Parameter
        + Member
        + SimpleArithmetic
        + Codec
        + Default
        + Copy
        + Clone
        + Hash
        + One
        + Zero
        + MaybeSerializeDeserialize
        + Eq
        + PartialEq
        + Ord;

    //type SimplifiedPropertyHash: From<Self::Hash> + EncodeLike + Hash + Default + PartialEq + Eq + Ord + Codec + MaybeSerializeDeserialize;

    /// Security/configuration constraints

    /// Type, representing min & max property name length constraints
    type PropertyNameLengthConstraint: Get<InputValidationLengthConstraint>;

    /// Type, representing min & max property description length constraints
    type PropertyDescriptionLengthConstraint: Get<InputValidationLengthConstraint>;

    /// Type, representing min & max class name length constraints
    type ClassNameLengthConstraint: Get<InputValidationLengthConstraint>;

    /// Type, representing min & max class description length constraints
    type ClassDescriptionLengthConstraint: Get<InputValidationLengthConstraint>;

    /// The maximum number of classes
    type MaxNumberOfClasses: Get<MaxNumber>;

    /// The maximum number of maintainers per class constraint
    type MaxNumberOfMaintainersPerClass: Get<MaxNumber>;

    /// The maximum number of curators per group constraint
    type MaxNumberOfCuratorsPerGroup: Get<MaxNumber>;

    /// The maximum number of schemas per class constraint
    type MaxNumberOfSchemasPerClass: Get<MaxNumber>;

    /// The maximum number of properties per class constraint
    type MaxNumberOfPropertiesPerSchema: Get<MaxNumber>;

    /// The maximum number of operations during single invocation of `transaction`
    type MaxNumberOfOperationsDuringAtomicBatching: Get<MaxNumber>;

    /// The maximum length of vector property value constarint
    type VecMaxLengthConstraint: Get<VecMaxLength>;

    /// The maximum length of text property value constarint
    type TextMaxLengthConstraint: Get<TextMaxLength>;

    /// The maximum length of text, that will be hashed property value constarint
    type HashedTextMaxLengthConstraint: Get<HashedTextMaxLength>;

    /// Entities creation constraint per class
    type MaxNumberOfEntitiesPerClass: Get<Self::EntityId>;

    /// Entities creation constraint per individual
    type IndividualEntitiesCreationLimit: Get<Self::EntityId>;
}

decl_storage! {
    trait Store for Module<T: Trait> as ContentDirectory {

        /// Map, representing ClassId -> Class relation
        pub ClassById get(class_by_id) config(): linked_map T::ClassId => Class<T>;

        /// Map, representing EntityId -> Entity relation
        pub EntityById get(entity_by_id) config(): map T::EntityId => Entity<T>;

        /// Map, representing  CuratorGroupId -> CuratorGroup relation
        pub CuratorGroupById get(curator_group_by_id) config(): map T::CuratorGroupId => CuratorGroup<T>;

        /// Used to enforce uniqueness of a property value across all Entities that have this property in a given Class.

        /// Mapping of class id and its property id to the respective entity id and property value hash.
        pub UniquePropertyValueHashes get(unique_property_value_hashes): double_map hasher(blake2_128) (T::ClassId, PropertyId), blake2_128(T::Hash) => ();

        /// Next runtime storage values used to maintain next id value, used on creation of respective curator groups, classes and entities

        pub NextClassId get(next_class_id) config(): T::ClassId;

        pub NextEntityId get(next_entity_id) config(): T::EntityId;

        pub NextCuratorGroupId get(next_curator_group_id) config(): T::CuratorGroupId;

        // The voucher associated with entity creation for a given class and controller.
        // Is updated whenever an entity is created in a given class by a given controller.
        // Constraint is updated by Root, an initial value comes from `ClassPermissions::default_entity_creation_voucher_upper_bound`.
        pub EntityCreationVouchers get(entity_creation_vouchers):
            double_map hasher(blake2_128) T::ClassId, blake2_128(EntityController<T>) => EntityCreationVoucher<T>;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        // ======
        // Next set of extrinsics can only be invoked by lead.
        // ======

        // Initializing events
        fn deposit_event() = default;

        /// Add new curator group to runtime storage
        pub fn add_curator_group(
            origin,
        ) -> dispatch::Result {

            // Ensure given origin is lead
            ensure_is_lead::<T>(origin)?;

            //
            // == MUTATION SAFE ==
            //

            let curator_group_id = Self::next_curator_group_id();

            // Insert empty curator group with `active` parameter set to false
            <CuratorGroupById<T>>::insert(curator_group_id, CuratorGroup::<T>::default());

            // Increment the next curator curator_group_id:
            <NextCuratorGroupId<T>>::mutate(|n| *n += T::CuratorGroupId::one());

            // Trigger event
            Self::deposit_event(RawEvent::CuratorGroupAdded(curator_group_id));
            Ok(())
        }

        /// Remove curator group under given `curator_group_id` from runtime storage
        pub fn remove_curator_group(
            origin,
            curator_group_id: T::CuratorGroupId,
        ) -> dispatch::Result {

            // Ensure given origin is lead
            ensure_is_lead::<T>(origin)?;

            // Ensure CuratorGroup under given curator_group_id exists
            let curator_group = Self::ensure_curator_group_exists(&curator_group_id)?;

            // We should previously ensure that curator_group  maintains no classes to be able to remove it
            curator_group.ensure_curator_group_maintains_no_classes()?;

            //
            // == MUTATION SAFE ==
            //


            // Remove curator group under given curator group id from runtime storage
            <CuratorGroupById<T>>::remove(curator_group_id);

            // Trigger event
            Self::deposit_event(RawEvent::CuratorGroupRemoved(curator_group_id));
            Ok(())
        }

        /// Set `is_active` status for curator group under given `curator_group_id`
        pub fn set_curator_group_status(
            origin,
            curator_group_id: T::CuratorGroupId,
            is_active: bool,
        ) -> dispatch::Result {

            // Ensure given origin is lead
            ensure_is_lead::<T>(origin)?;

            // Ensure curator group under provided curator_group_id already exist
            Self::ensure_curator_group_under_given_id_exists(&curator_group_id)?;

            //
            // == MUTATION SAFE ==
            //

            // Set `is_active` status for curator group under given `curator_group_id`
            <CuratorGroupById<T>>::mutate(curator_group_id, |curator_group| {
                curator_group.set_status(is_active)
            });

            // Trigger event
            Self::deposit_event(RawEvent::CuratorGroupStatusSet(curator_group_id, is_active));
            Ok(())
        }

        /// Add curator to curator group under given `curator_group_id`
        pub fn add_curator_to_group(
            origin,
            curator_group_id: T::CuratorGroupId,
            curator_id: T::CuratorId,
        ) -> dispatch::Result {

            // Ensure given origin is lead
            ensure_is_lead::<T>(origin)?;

            // Ensure curator group under provided curator_group_id already exist, retrieve corresponding one
            let curator_group = Self::ensure_curator_group_exists(&curator_group_id)?;

            // Ensure max number of curators per group limit not reached yet
            curator_group.ensure_max_number_of_curators_limit_not_reached()?;

            //
            // == MUTATION SAFE ==
            //

            // Insert curator_id into curator_group under given curator_group_id
            <CuratorGroupById<T>>::mutate(curator_group_id, |curator_group| {
                curator_group.get_curators_mut().insert(curator_id);
            });

            // Trigger event
            Self::deposit_event(RawEvent::CuratorAdded(curator_group_id, curator_id));
            Ok(())
        }

        /// Remove curator from a given curator group
        pub fn remove_curator_from_group(
            origin,
            curator_group_id: T::CuratorGroupId,
            curator_id: T::CuratorId,
        ) -> dispatch::Result {

            // Ensure given origin is lead
            ensure_is_lead::<T>(origin)?;

            // Ensure curator group under provided curator_group_id already exist, retrieve corresponding one
            let curator_group = Self::ensure_curator_group_exists(&curator_group_id)?;

            // Ensure curator under provided curator_id is CuratorGroup member
            curator_group.ensure_curator_in_group_exists(&curator_id)?;

            //
            // == MUTATION SAFE ==
            //

            // Remove curator_id from curator_group under given curator_group_id
            <CuratorGroupById<T>>::mutate(curator_group_id, |curator_group| {
                curator_group.get_curators_mut().remove(&curator_id);
            });

            // Trigger event
            Self::deposit_event(RawEvent::CuratorRemoved(curator_group_id, curator_id));
            Ok(())
        }

        /// Add curator group under given `curator_group_id` as `Class` maintainer
        pub fn add_maintainer_to_class(
            origin,
            class_id: T::ClassId,
            curator_group_id: T::CuratorGroupId,
        ) -> dispatch::Result {

            // Ensure given origin is lead
            ensure_is_lead::<T>(origin)?;

            // Ensure Class under provided class_id exist, retrieve corresponding one
            let class = Self::ensure_known_class_id(class_id)?;

            // Ensure CuratorGroup under provided curator_group_id exist, retrieve corresponding one
            Self::ensure_curator_group_under_given_id_exists(&curator_group_id)?;

            // Ensure the max number of maintainers per Class limit not reached
            let class_permissions = class.get_permissions_ref();

            // Ensure max number of maintainers per Class constraint satisfied
            Self::ensure_maintainers_limit_not_reached(class_permissions.get_maintainers())?;

            // Ensure maintainer under provided curator_group_id is not added to the Class maintainers set yet
            class_permissions.ensure_maintainer_does_not_exist(&curator_group_id)?;

            //
            // == MUTATION SAFE ==
            //

            // Insert `curator_group_id` into `maintainers` set, associated with given `Class`
            <ClassById<T>>::mutate(class_id, |class|
                class.get_permissions_mut().get_maintainers_mut().insert(curator_group_id)
            );

            // Increment the number of classes, curator group under given `curator_group_id` maintains
            <CuratorGroupById<T>>::mutate(curator_group_id, |curator_group| {
                curator_group.increment_number_of_classes_maintained_count();
            });

            // Trigger event
            Self::deposit_event(RawEvent::MaintainerAdded(class_id, curator_group_id));
            Ok(())
        }

        /// Remove curator group under given `curator_group_id` from `Class` maintainers set
        pub fn remove_maintainer_from_class(
            origin,
            class_id: T::ClassId,
            curator_group_id: T::CuratorGroupId,
        ) -> dispatch::Result {

            // Ensure given origin is lead
            ensure_is_lead::<T>(origin)?;

            // Ensure Class under given id exists, return corresponding one
            let class = Self::ensure_known_class_id(class_id)?;

            // Ensure maintainer under provided curator_group_id was previously added
            // to the maintainers set, associated with corresponding Class
            class.get_permissions_ref().ensure_maintainer_exists(&curator_group_id)?;

            //
            // == MUTATION SAFE ==
            //

            // Remove `curator_group_id` from `maintainers` set, associated with given `Class`
            <ClassById<T>>::mutate(class_id, |class|
                class.get_permissions_mut().get_maintainers_mut().remove(&curator_group_id)
            );

            // Decrement the number of classes, curator group under given `curator_group_id` maintains
            <CuratorGroupById<T>>::mutate(curator_group_id, |curator_group| {
                curator_group.decrement_number_of_classes_maintained_count();
            });

            // Trigger event
            Self::deposit_event(RawEvent::MaintainerRemoved(class_id, curator_group_id));
            Ok(())
        }

        /// Updates or creates new `EntityCreationVoucher` for given `EntityController` with individual limit
        pub fn update_entity_creation_voucher(
            origin,
            class_id: T::ClassId,
            controller: EntityController<T>,
            maximum_entities_count: T::EntityId
        ) -> dispatch::Result {

            // Ensure given origin is lead
            ensure_is_lead::<T>(origin)?;

            // Ensure Class under given id exists, return corresponding one
            Self::ensure_known_class_id(class_id)?;

            // Ensure maximum_entities_count does not exceed individual entities creation limit
            Self::ensure_valid_number_of_class_entities_per_actor_constraint(maximum_entities_count)?;

            // Check voucher existance
            let voucher_exists = <EntityCreationVouchers<T>>::exists(class_id, &controller);

            //
            // == MUTATION SAFE ==
            //

            if voucher_exists {

                // Set new maximum_entities_count limit for selected voucher
                let mut entity_creation_voucher = Self::entity_creation_vouchers(class_id, &controller);

                entity_creation_voucher.set_maximum_entities_count(maximum_entities_count);

                <EntityCreationVouchers<T>>::insert(class_id, controller.clone(), entity_creation_voucher.clone());

                // Trigger event
                Self::deposit_event(RawEvent::EntityCreationVoucherUpdated(controller, entity_creation_voucher))
            } else {
                // Create new EntityCreationVoucher instance with provided maximum_entities_count
                let entity_creation_voucher = EntityCreationVoucher::new(maximum_entities_count);

                // Add newly created `EntityCreationVoucher` into `EntityCreationVouchers`
                // runtime storage under given `class_id`, `controller` key
                <EntityCreationVouchers<T>>::insert(class_id, controller.clone(), entity_creation_voucher.clone());

                // Trigger event
                Self::deposit_event(RawEvent::EntityCreationVoucherCreated(controller, entity_creation_voucher));
            }

            Ok(())
        }

        /// Create new `Class` with provided parameters
        pub fn create_class(
            origin,
            name: Vec<u8>,
            description: Vec<u8>,
            class_permissions: ClassPermissions<T>,
            maximum_entities_count: T::EntityId,
            default_entity_creation_voucher_upper_bound: T::EntityId
        ) -> dispatch::Result {

            // Ensure given origin is lead
            ensure_is_lead::<T>(origin)?;

            // Ensure, that all entities creation limits, defined for a given Class, are valid
            Self::ensure_entities_creation_limits_are_valid(maximum_entities_count, default_entity_creation_voucher_upper_bound)?;

            // Ensure max number of classes limit not reached
            Self::ensure_class_limit_not_reached()?;

            // Ensure ClassNameLengthConstraint conditions satisfied
            Self::ensure_class_name_is_valid(&name)?;

            // Ensure ClassDescriptionLengthConstraint conditions satisfied
            Self::ensure_class_description_is_valid(&description)?;

            // Perform required checks to ensure class_maintainers under provided class_permissions are valid
            let class_maintainers = class_permissions.get_maintainers();
            Self::ensure_class_maintainers_are_valid(class_maintainers)?;

            //
            // == MUTATION SAFE ==
            //

            // Create new Class instance from provided values
            let class = Class::new(
                class_permissions, name, description, maximum_entities_count, default_entity_creation_voucher_upper_bound
            );

            let class_id = Self::next_class_id();

            // Add new `Class` to runtime storage
            <ClassById<T>>::insert(&class_id, class);

            // Increment the next class id:
            <NextClassId<T>>::mutate(|n| *n += T::ClassId::one());

            // Trigger event
            Self::deposit_event(RawEvent::ClassCreated(class_id));
            Ok(())
        }

        /// Update `ClassPermissions` under specific `class_id`
        pub fn update_class_permissions(
            origin,
            class_id: T::ClassId,
            updated_any_member: Option<bool>,
            updated_entity_creation_blocked: Option<bool>,
            updated_all_entity_property_values_locked: Option<bool>,
            updated_maintainers: Option<BTreeSet<T::CuratorGroupId>>,
        ) -> dispatch::Result {

            // Ensure given origin is lead
            ensure_is_lead::<T>(origin)?;

            // Ensure Class under given id exists, return corresponding one
            let class = Self::ensure_known_class_id(class_id)?;

            // Perform required checks to ensure class_maintainers are valid
            if let Some(ref updated_maintainers) = updated_maintainers {
                Self::ensure_class_maintainers_are_valid(updated_maintainers)?;
            }

            //
            // == MUTATION SAFE ==
            //

            let class_permissions = class.get_permissions();

            // Make updated class_permissions from parameters provided
            let updated_class_permissions = Self::make_updated_class_permissions(
                class_permissions, updated_any_member, updated_entity_creation_blocked,
                updated_all_entity_property_values_locked, updated_maintainers
            );

            // If class_permissions update has been performed
            if let Some(updated_class_permissions) = updated_class_permissions  {

                // Update `class_permissions` under given class id
                <ClassById<T>>::mutate(class_id, |class| {
                    class.update_permissions(updated_class_permissions)
                });

                // Trigger event
                Self::deposit_event(RawEvent::ClassPermissionsUpdated(class_id));
            }

            Ok(())
        }

        /// Create new class schema from existing property ids and new properties
        pub fn add_class_schema(
            origin,
            class_id: T::ClassId,
            existing_properties: BTreeSet<PropertyId>,
            new_properties: Vec<Property<T>>
        ) -> dispatch::Result {

            // Ensure given origin is lead
            ensure_is_lead::<T>(origin)?;

            // Ensure Class under given id exists, return corresponding one
            let class = Self::ensure_known_class_id(class_id)?;

            // Ensure Schemas limit per Class not reached
            class.ensure_schemas_limit_not_reached()?;

            // Ensure both existing and new properties for future Schema are not empty
            Self::ensure_non_empty_schema(&existing_properties, &new_properties)?;

            // Ensure max number of properties per Schema limit not reached
            class.ensure_properties_limit_not_reached(&new_properties)?;

            // Complete all checks to ensure all provided new_properties are valid
            Self::ensure_all_properties_are_valid(&new_properties)?;

            // Id of next Class Schema being added
            let schema_id = class.get_schemas().len() as SchemaId;

            let class_properties = class.get_properties();

            // Ensure all Property names are unique within Class
            Self::ensure_all_property_names_are_unique(&class_properties, &new_properties)?;

            // Ensure existing_properties are valid indices of properties, corresponding to chosen Class
            Self::ensure_schema_properties_are_valid_indices(&existing_properties, &class_properties)?;

            //
            // == MUTATION SAFE ==
            //

            // Create `Schema` instance from existing and new property ids
            let schema = Self::create_class_schema(existing_properties, &class_properties, &new_properties);

            // Update class properties after new `Schema` added
            let updated_class_properties = Self::make_updated_class_properties(class_properties, new_properties);

            // Update Class properties and schemas
            <ClassById<T>>::mutate(class_id, |class| {
                class.set_properties(updated_class_properties);
                class.get_schemas_mut().push(schema);
            });

            // Trigger event
            Self::deposit_event(RawEvent::ClassSchemaAdded(class_id, schema_id));

            Ok(())
        }

        /// Update `schema_status` under specific `schema_id` in `Class`
        pub fn update_class_schema_status(
            origin,
            class_id: T::ClassId,
            schema_id: SchemaId,
            schema_status: bool
        ) -> dispatch::Result {

            // Ensure given origin is lead
            ensure_is_lead::<T>(origin)?;

            // Ensure Class under given id exists, return corresponding one
            let class = Self::ensure_known_class_id(class_id)?;

            // Ensure Class already contains schema under provided schema_id
            class.ensure_schema_id_exists(schema_id)?;

            //
            // == MUTATION SAFE ==
            //

            // Update class schema status
            <ClassById<T>>::mutate(class_id, |class| {
                class.update_schema_status(schema_id, schema_status)
            });

            // Trigger event
            Self::deposit_event(RawEvent::ClassSchemaStatusUpdated(class_id, schema_id, schema_status));
            Ok(())
        }

        /// Update entity permissions
        pub fn update_entity_permissions(
            origin,
            entity_id: T::EntityId,
            updated_frozen: Option<bool>,
            updated_referenceable: Option<bool>
        ) -> dispatch::Result {

            // Ensure given origin is lead
            ensure_is_lead::<T>(origin)?;

            // Ensure Entity under given id exists, return corresponding one
            let entity = Self::ensure_known_entity_id(entity_id)?;

            //
            // == MUTATION SAFE ==
            //

            // Make updated entity_permissions from parameters provided
            let entity_permissions = entity.get_permissions();

            let updated_entity_permissions =
                Self::make_updated_entity_permissions(entity_permissions, updated_frozen, updated_referenceable);

            // Update entity permissions under given entity id
            if let Some(updated_entity_permissions) = updated_entity_permissions {

                <EntityById<T>>::mutate(entity_id, |entity| {
                    entity.update_permissions(updated_entity_permissions)
                });

                // Trigger event
                Self::deposit_event(RawEvent::EntityPermissionsUpdated(entity_id));
            }
            Ok(())
        }

        /// Transfer ownership to new `EntityController` for `Entity` under given `entity_id`
        /// `new_property_value_references_with_same_owner_flag_set` should be provided manually
        pub fn transfer_entity_ownership(
            origin,
            entity_id: T::EntityId,
            new_controller: EntityController<T>,
            new_property_value_references_with_same_owner_flag_set: BTreeMap<PropertyId, InputPropertyValue<T>>
        ) -> dispatch::Result {

            // Ensure given origin is lead
            ensure_is_lead::<T>(origin)?;

            // Ensure Entity under given entity_id exists, retrieve corresponding Entity & Class
            let (entity, class) = Self::ensure_known_entity_and_class(entity_id)?;

            // Ensure provided new_entity_controller is not equal to current one
            entity.get_permissions_ref().ensure_controllers_are_not_equal(&new_controller)?;

            // Ensure any inbound InputPropertyValue::Reference with same_owner flag set points to the given Entity
            entity.ensure_inbound_same_owner_rc_is_zero()?;

            let class_properties = class.get_properties();

            let class_id = entity.get_class_id();

            let entity_property_values = entity.get_values();

            // Create wrapper structure from provided entity_property_values and their corresponding Class properties
            let values_for_existing_properties = match StoredValuesForExistingProperties::from(&class_properties, &entity_property_values) {
                Ok(values_for_existing_properties) => values_for_existing_properties,
                Err(e) => {
                    debug_assert!(false, "Should not fail! {:?}", e);
                    return Err(e)
                }
            };

            // Filter provided values_for_existing_properties, leaving only `Reference`'s with `SameOwner` flag set
            // Retrieve the set of corresponding property ids
            let entity_property_id_references_with_same_owner_flag_set =
                Self::get_property_id_references_with_same_owner_flag_set(values_for_existing_properties);

            // Ensure all ids of provided `new_property_value_references_with_same_owner_flag_set`
            // corresponding to property ids of respective Class Property references with same owner flag set
            Self::ensure_only_reference_ids_with_same_owner_flag_set_provided(
                &entity_property_id_references_with_same_owner_flag_set,
                &new_property_value_references_with_same_owner_flag_set
            )?;

            // Retrieve ids of all entity property values, that are references with same owner flag set and which are not provided
            // in new property value references with same owner flag set
            let unused_property_id_references_with_same_owner_flag_set = Self::compute_unused_property_ids(
                &new_property_value_references_with_same_owner_flag_set, &entity_property_id_references_with_same_owner_flag_set
            );

            // Perform checks to ensure all required property_values under provided unused_schema_property_ids provided
            Self::ensure_all_required_properties_provided(&class_properties, &unused_property_id_references_with_same_owner_flag_set)?;

            // Create wrapper structure from provided new_property_value_references_with_same_owner_flag_set and their corresponding Class properties
            let new_values_for_existing_properties = InputValuesForExistingProperties::from(
                &class_properties, &new_property_value_references_with_same_owner_flag_set
            )?;

            // Ensure all provided `new_property_value_references_with_same_owner_flag_set` are valid
            Self::ensure_are_valid_references_with_same_owner_flag_set(
                new_values_for_existing_properties, &new_controller
            )?;

            let new_output_property_value_references_with_same_owner_flag_set = Self::make_output_property_values(new_property_value_references_with_same_owner_flag_set);

            // Compute StoredPropertyValues, which respective Properties have unique flag set
            // (skip PropertyIds, which respective property values under this Entity are default and non required)
            let new_output_values_for_existing_properties =
                StoredValuesForExistingProperties::from(&class_properties, &new_output_property_value_references_with_same_owner_flag_set)?;

            // Compute new unique property value hashes.
            // Ensure new property value hashes with `unique` flag set are `unique` on `Class` level
            let new_unique_hashes = Self::ensure_new_property_values_respect_uniquness(
                class_id, new_output_values_for_existing_properties,
            )?;

            //
            // == MUTATION SAFE ==
            //

            // Used to compute old unique hashes, that should be substituted with new ones.
            let old_unique_hashes =
                Self::compute_old_unique_hashes(&new_output_property_value_references_with_same_owner_flag_set, &entity_property_values);

            // Add property values, that should be unique on Class level
            Self::add_unique_property_value_hashes(class_id, new_unique_hashes);

            // Remove unique hashes, that were substituted with new ones.
            Self::remove_unique_property_value_hashes(class_id, old_unique_hashes);

            // Make updated entity_property_values from parameters provided
            let entity_property_values_updated =
                    Self::make_updated_property_value_references_with_same_owner_flag_set(
                        unused_property_id_references_with_same_owner_flag_set, &entity_property_values,
                        &new_output_property_value_references_with_same_owner_flag_set,
                    );

            // Transfer entity ownership
            let entities_inbound_rcs_delta = if let Some(entity_property_values_updated) = entity_property_values_updated {


                // Calculate entities reference counter side effects for current operation
                let entities_inbound_rcs_delta =
                    Self::get_updated_inbound_rcs_delta(
                        entity_id, class_properties, entity_property_values, new_output_property_value_references_with_same_owner_flag_set
                    )?;

                // Update InboundReferenceCounter, based on previously calculated ReferenceCounterSideEffects, for each Entity involved
                Self::update_entities_rcs(&entities_inbound_rcs_delta);

                <EntityById<T>>::mutate(entity_id, |entity| {

                    // Update current Entity property values with updated ones
                    entity.set_values(entity_property_values_updated);

                    // Set up new controller for the current Entity instance
                    entity.get_permissions_mut().set_conroller(new_controller.clone());
                });

                entities_inbound_rcs_delta
            } else {
                // Set up new controller for the current Entity instance
                <EntityById<T>>::mutate(entity_id, |entity| {
                    entity.get_permissions_mut().set_conroller(new_controller.clone());
                });

                None
            };

            // Trigger event
            Self::deposit_event(RawEvent::EntityOwnershipTransfered(entity_id, new_controller, entities_inbound_rcs_delta));

            Ok(())
        }

        // ======
        // The next set of extrinsics can be invoked by anyone who can properly sign for provided value of `Actor<T>`.
        // ======

        /// Create an entity.
        /// If someone is making an entity of this class for first time,
        /// then a voucher is also added with the class limit as the default limit value.
        pub fn create_entity(
            origin,
            class_id: T::ClassId,
            actor: Actor<T>,
        ) -> dispatch::Result {

            let account_id = ensure_signed(origin)?;

            // Ensure Class under given id exists, return corresponding one
            let class = Self::ensure_class_exists(class_id)?;

            // Ensure maximum entities limit per class not reached
            class.ensure_maximum_entities_count_limit_not_reached()?;

            let class_permissions = class.get_permissions_ref();

            // Ensure entities creation is not blocked on Class level
            class_permissions.ensure_entity_creation_not_blocked()?;

            // Ensure actor can create entities
            class_permissions.ensure_can_create_entities(&account_id, &actor)?;

            let entity_controller = EntityController::from_actor(&actor);

            // Check if entity creation voucher exists
            let voucher_exists = if <EntityCreationVouchers<T>>::exists(class_id, &entity_controller) {

                // Ensure voucher limit not reached
                Self::entity_creation_vouchers(class_id, &entity_controller).ensure_voucher_limit_not_reached()?;
                true
            } else {
                false
            };

            //
            // == MUTATION SAFE ==
            //

            // Create voucher, update if exists

            if voucher_exists {

                // Increment number of created entities count, if specified voucher already exist
                <EntityCreationVouchers<T>>::mutate(class_id, &entity_controller, |entity_creation_voucher| {
                    entity_creation_voucher.increment_created_entities_count()
                });
            } else {

                // Create new voucher for given entity creator with default limit
                let mut entity_creation_voucher = EntityCreationVoucher::new(class.get_default_entity_creation_voucher_upper_bound());

                // Increase created entities count by 1 to maintain valid entity_creation_voucher state after following Entity added
                entity_creation_voucher.increment_created_entities_count();
                <EntityCreationVouchers<T>>::insert(class_id, entity_controller.clone(), entity_creation_voucher);
            }

            // Create new entity

            let entity_id = Self::next_entity_id();

            let new_entity = Entity::<T>::new(
                entity_controller,
                class_id,
                BTreeSet::new(),
                BTreeMap::new(),
            );

            // Save newly created entity:
            EntityById::insert(entity_id, new_entity);

            // Increment the next entity id:
            <NextEntityId<T>>::mutate(|n| *n += T::EntityId::one());

            // Increment number of entities, associated with this class
            <ClassById<T>>::mutate(class_id, |class| {
                class.increment_entities_count();
            });

            // Trigger event
            Self::deposit_event(RawEvent::EntityCreated(actor, entity_id));
            Ok(())
        }

        /// Remove `Entity` under provided `entity_id`
        pub fn remove_entity(
            origin,
            actor: Actor<T>,
            entity_id: T::EntityId,
        ) -> dispatch::Result {

            // Retrieve Entity and EntityAccessLevel for the actor, attemting to perform operation
            let (class, entity, access_level) = Self::ensure_class_entity_and_access_level(origin, entity_id, &actor)?;

            // Ensure actor with given EntityAccessLevel can remove entity
            EntityPermissions::<T>::ensure_group_can_remove_entity(access_level)?;

            // Ensure any inbound InputPropertyValue::Reference points to the given Entity
            entity.ensure_rc_is_zero()?;

            let class_properties = class.get_properties();

            let class_id = entity.get_class_id();

            let entity_values = entity.get_values();

            let unique_property_value_hashes = match StoredValuesForExistingProperties::from(&class_properties, &entity_values) {
                Ok(values_for_existing_properties) => values_for_existing_properties.compute_unique_hashes(),
                Err(e) => {
                    debug_assert!(false, "Should not fail! {:?}", e);
                    return Err(e)
                }
            };

            //
            // == MUTATION SAFE ==
            //

            // Remove property value entries, that should be unique on Class level
            Self::remove_unique_property_value_hashes(class_id, unique_property_value_hashes);

            // Remove entity
            <EntityById<T>>::remove(entity_id);

            // Decrement class entities counter
            <ClassById<T>>::mutate(class_id, |class| class.decrement_entities_count());

            let entity_controller = EntityController::<T>::from_actor(&actor);

            // Decrement entity_creation_voucher after entity removal perfomed
            <EntityCreationVouchers<T>>::mutate(class_id, entity_controller, |entity_creation_voucher| {
                entity_creation_voucher.decrement_created_entities_count();
            });

            // Trigger event
            Self::deposit_event(RawEvent::EntityRemoved(actor, entity_id));
            Ok(())
        }

        /// Add schema support to entity under given `schema_id` and provided `property_values`
        pub fn add_schema_support_to_entity(
            origin,
            actor: Actor<T>,
            entity_id: T::EntityId,
            schema_id: SchemaId,
            new_property_values: BTreeMap<PropertyId, InputPropertyValue<T>>
        ) -> dispatch::Result {

            // Retrieve Class, Entity and ensure given have access to the Entity under given entity_id
            let (class, entity, _) = Self::ensure_class_entity_and_access_level(origin, entity_id, &actor)?;

            // Ensure Class Schema under given index exists, return corresponding Schema
            let schema = class.ensure_schema_exists(schema_id)?.to_owned();

            let class_properties = class.get_properties();

            // Create wrapper structure from provided new_property_values and their corresponding Class properties
            let new_values_for_existing_properties = InputValuesForExistingProperties::from(&class_properties, &new_property_values)?;

            // Ensure Schema under given id is not added to given Entity yet
            entity.ensure_schema_id_is_not_added(schema_id)?;

            // Ensure provided new_property_values are not added to the Entity values map yet
            entity.ensure_property_values_are_not_added(&new_property_values)?;

            // Ensure provided schema can be added to the Entity
            schema.ensure_is_active()?;

            // Ensure all provided new property values are for properties in the given schema
            schema.ensure_has_properties(&new_property_values)?;

            // Retrieve Schema property ids, which are not provided in new_property_values
            let unused_schema_property_ids = Self::compute_unused_property_ids(&new_property_values, schema.get_properties());

            // Perform checks to ensure all required property_values under provided unused_schema_property_ids provided
            Self::ensure_all_required_properties_provided(&class_properties, &unused_schema_property_ids)?;

            // Ensure all property_values under given Schema property ids are valid
            let entity_controller = entity.get_permissions_ref().get_controller();

            // Validate all values, provided in new_values_for_existing_properties,
            // against the type of its Property and check any additional constraints
            Self::ensure_property_values_are_valid(&entity_controller, &new_values_for_existing_properties)?;

            let class_id = entity.get_class_id();

            let entity_property_values = entity.get_values();

            let new_output_property_values = Self::make_output_property_values(new_property_values);

            // Compute updated entity values, after new schema support added
            let entity_values_updated = Self::make_updated_entity_property_values(
                schema, entity_property_values, &new_output_property_values
            );

            let new_output_values_for_existing_properties = StoredValuesForExistingProperties::from(&class_properties, &new_output_property_values)?;

            // Retrieve StoredPropertyValues, which respective Properties have unique flag set
            // (skip PropertyIds, which respective property values under this Entity are default and non required)
            let new_unique_property_value_hashes = new_output_values_for_existing_properties.compute_unique_hashes();

            // Ensure all provided Properties with unique flag set are unique on Class level
            Self::ensure_property_value_hashes_unique_option_satisfied(class_id, &new_unique_property_value_hashes)?;

            //
            // == MUTATION SAFE ==
            //

            // Add property value hashes, that should be unique on Class level
            Self::add_unique_property_value_hashes(class_id, new_unique_property_value_hashes);

            // Calculate entities reference counter side effects for current operation
            let entities_inbound_rcs_delta = Self::calculate_entities_inbound_rcs_delta(
                entity_id, new_output_values_for_existing_properties, DeltaMode::Increment
            );

            // Update InboundReferenceCounter, based on previously calculated entities_inbound_rcs_delta, for each Entity involved
            Self::update_entities_rcs(&entities_inbound_rcs_delta);

            // Add schema support to `Entity` under given `entity_id`
            <EntityById<T>>::mutate(entity_id, |entity| {

                // Add a new schema to the list of schemas supported by this entity.
                entity.get_supported_schemas_mut().insert(schema_id);

                // Update entity values only if new properties have been added.
                if entity_values_updated.len() > entity.get_values_ref().len() {
                    entity.set_values(entity_values_updated);
                }
            });

            // Trigger event
            Self::deposit_event(RawEvent::EntitySchemaSupportAdded(actor, entity_id, schema_id, entities_inbound_rcs_delta));
            Ok(())
        }

        /// Update `Entity` `InputPropertyValue`'s with provided ones
        pub fn update_entity_property_values(
            origin,
            actor: Actor<T>,
            entity_id: T::EntityId,
            new_property_values: BTreeMap<PropertyId, InputPropertyValue<T>>
        ) -> dispatch::Result {

            // Retrieve Class, Entity and EntityAccessLevel for the actor, attemting to perform operation
            let (class, entity, access_level) = Self::ensure_class_entity_and_access_level(origin, entity_id, &actor)?;

            // Ensure property values were not locked on Class level
            class.ensure_property_values_unlocked()?;

            let entity_values_ref = entity.get_values_ref();

            // Filter new_property_values, that are identical to entity_property_values.
            // Get `new_property_values`, that are not in `entity_property_values`
            let new_property_values = Self::try_filter_identical_property_values(entity_values_ref, new_property_values);

            // Ensure all provided new_property_values are already added to the current Entity instance
            Self::ensure_all_property_values_are_already_added(entity_values_ref, &new_property_values)?;

            let class_properties = class.get_properties();

            // Create wrapper structure from new_property_values and their corresponding Class properties
            let new_values_for_existing_properties = InputValuesForExistingProperties::from(&class_properties, &new_property_values)?;

            // Ensure all provided property values are unlocked for the actor with given access_level
            Self::ensure_all_property_values_are_unlocked_from(&new_values_for_existing_properties, access_level)?;

            let entity_controller = entity.get_permissions_ref().get_controller();

            // Validate all values, provided in values_for_existing_properties,
            // against the type of its Property and check any additional constraints
            Self::ensure_property_values_are_valid(&entity_controller, &new_values_for_existing_properties)?;

            let class_id = entity.get_class_id();

            // Get current property values of an Entity

            let entity_property_values = entity.get_values();

            let new_output_property_values = Self::make_output_property_values(new_property_values);

            // Compute StoredPropertyValues, which respective Properties have unique flag set
            // (skip PropertyIds, which respective property values under this Entity are default and non required)
            let new_output_values_for_existing_properties =
                StoredValuesForExistingProperties::from(&class_properties, &new_output_property_values)?;

            // Compute new unique property value hashes.
            // Ensure new property value hashes with `unique` flag set are `unique` on `Class` level
            let new_unique_hashes = Self::ensure_new_property_values_respect_uniquness(
                class_id, new_output_values_for_existing_properties,
            )?;

            //
            // == MUTATION SAFE ==
            //

            // Used to compute old unique hashes, that should be substituted with new ones.
            let old_unique_hashes =
                Self::compute_old_unique_hashes(&new_output_property_values, &entity_property_values);

            // Add property value hashes, that should be unique on Class level
            Self::add_unique_property_value_hashes(class_id, new_unique_hashes);

            // Remove unique hashes, that were substituted with new ones. (if some).
            Self::remove_unique_property_value_hashes(class_id, old_unique_hashes);

            // Make updated entity_property_values from current entity_property_values and new_output_property_values provided
            let entity_property_values_updated =
                Self::make_updated_property_values(&entity_property_values, &new_output_property_values);

            // If property values should be updated
            if let Some(entity_property_values_updated) = entity_property_values_updated {

                // Calculate entities reference counter side effects for current operation (should always be safe)
                let entities_inbound_rcs_delta =
                    Self::get_updated_inbound_rcs_delta(entity_id, class_properties, entity_property_values, new_output_property_values)?;

                // Update InboundReferenceCounter, based on previously calculated entities_inbound_rcs_delta, for each Entity involved
                Self::update_entities_rcs(&entities_inbound_rcs_delta);

                // Update entity property values
                <EntityById<T>>::mutate(entity_id, |entity| {
                    entity.set_values(entity_property_values_updated);
                });

                // Trigger event
                Self::deposit_event(RawEvent::EntityPropertyValuesUpdated(actor, entity_id, entities_inbound_rcs_delta));
            }

            Ok(())
        }

        /// Clear `PropertyValueVec` under given `entity_id` & `in_class_schema_property_id`
        pub fn clear_entity_property_vector(
            origin,
            actor: Actor<T>,
            entity_id: T::EntityId,
            in_class_schema_property_id: PropertyId
        ) -> dispatch::Result {

            // Retrieve Class, Entity and EntityAccessLevel for the actor, attemting to perform operation
            let (class, entity, access_level) = Self::ensure_class_entity_and_access_level(origin, entity_id, &actor)?;

            // Ensure Property under given PropertyId is unlocked from actor with given EntityAccessLevel
            // Retrieve corresponding Property by value
            let property = class.ensure_class_property_type_unlocked_from(
                in_class_schema_property_id,
                access_level,
            )?;

            // Ensure PropertyValue under given in_class_schema_property_id is Vector
            let property_value_vector =
                entity.ensure_property_value_is_vec(in_class_schema_property_id)?;

            // Calculate side effects for clear_property_vector operation, based on property_value_vector provided and its respective property.
            let entities_inbound_rcs_delta = Self::make_side_effects_for_clear_property_vector_operation(&property_value_vector, &property);

            // Clear property_value_vector.
            let empty_property_value_vector = Self::clear_property_vector(property_value_vector.clone());

            let class_id = entity.get_class_id();

            // Compute old and new vec unique property value hash.
            // Ensure new property value hash with `unique` flag set is `unique` on `Class` level
            let vec_property_value_hashes = if property.unique {
                Some(
                    Self::ensure_vec_property_value_hashes(class_id, in_class_schema_property_id, &empty_property_value_vector, property_value_vector)?
                )
            } else {
                None
            };

            //
            // == MUTATION SAFE ==
            //

            if let Some((new_property_value_hash, old_property_value_hash)) = vec_property_value_hashes {
                // Add property value hash, that should be unique on `Class` level
                Self::add_unique_property_value_hash(class_id, in_class_schema_property_id, new_property_value_hash);

                // Remove property value hash, that should be unique on `Class` level
                Self::remove_unique_property_value_hash(class_id, in_class_schema_property_id, old_property_value_hash);
            }

            // Decrease reference counters of involved entities (if some)
            Self::update_entities_rcs(&entities_inbound_rcs_delta);

            // Insert empty_property_value_vector into entity_property_values mapping at in_class_schema_property_id.
            // Retrieve updated entity_property_values
            let entity_values_updated = Self::insert_at_in_class_schema_property_id(
                entity.get_values(), in_class_schema_property_id, empty_property_value_vector
            );

            // Update entity property values
            <EntityById<T>>::mutate(entity_id, |entity| {
                entity.set_values(entity_values_updated);
            });

            // Trigger event
            Self::deposit_event(
                RawEvent::VectorCleared(
                    actor, entity_id, in_class_schema_property_id, entities_inbound_rcs_delta
                )
            );

            Ok(())
        }

        /// Remove value at given `index_in_property_vector`
        /// from `PropertyValueVec` under in_class_schema_property_id
        pub fn remove_at_entity_property_vector(
            origin,
            actor: Actor<T>,
            entity_id: T::EntityId,
            in_class_schema_property_id: PropertyId,
            index_in_property_vector: VecMaxLength,
            nonce: T::Nonce
        ) -> dispatch::Result {

            // Retrieve Class, Entity and EntityAccessLevel for the actor, attemting to perform operation
            let (class, entity, access_level) = Self::ensure_class_entity_and_access_level(origin, entity_id, &actor)?;

            // Ensure Property under given PropertyId is unlocked from actor with given EntityAccessLevel
            // Retrieve corresponding Property by value
            let property = class.ensure_class_property_type_unlocked_from(
                in_class_schema_property_id,
                access_level,
            )?;

            // Ensure InputPropertyValue under given in_class_schema_property_id is Vector
            let property_value_vector =
                entity.ensure_property_value_is_vec(in_class_schema_property_id)?;

            // Ensure `VecInputPropertyValue` nonce is equal to the provided one.
            // Used to to avoid possible data races, when performing vector specific operations
            property_value_vector.ensure_nonce_equality(nonce)?;

            // Ensure, provided index_in_property_vec is valid index of VecInputValue
            property_value_vector
                .ensure_index_in_property_vector_is_valid(index_in_property_vector)?;

            let involved_entity_id = property_value_vector
                .get_vec_value_ref()
                .get_involved_entities()
                .and_then(|involved_entities| involved_entities.get(index_in_property_vector as usize).copied());

            // Remove value at in_class_schema_property_id in property value vector
            // Get VecInputPropertyValue wrapped in InputPropertyValue
            let property_value_vector_updated = Self::remove_at_index_in_property_vector(
                property_value_vector.clone(), index_in_property_vector
            );

            let class_id = entity.get_class_id();

            // Compute old and new vec unique property value hash.
            // Ensure new property value hash with `unique` flag set is `unique` on `Class` level
            let vec_property_value_hashes = if property.unique {
                Some(
                    Self::ensure_vec_property_value_hashes(class_id, in_class_schema_property_id, &property_value_vector_updated, property_value_vector)?
                )
            } else {
                None
            };

            //
            // == MUTATION SAFE ==
            //

            if let Some((new_property_value_hash, old_property_value_hash)) = vec_property_value_hashes {
                // Add property value hash, that should be unique on `Class` level
                Self::add_unique_property_value_hash(class_id, in_class_schema_property_id, new_property_value_hash);

                // Remove property value hash, that should be unique on `Class` level
                Self::remove_unique_property_value_hash(class_id, in_class_schema_property_id, old_property_value_hash);
            }

            // Insert updated propery value into entity_property_values mapping at in_class_schema_property_id.
            let entity_values_updated = Self::insert_at_in_class_schema_property_id(
                entity.get_values(), in_class_schema_property_id, property_value_vector_updated
            );

            let involved_entity_and_side_effect = if let Some(involved_entity_id) = involved_entity_id {
                // Decrease reference counter of involved entity (if some)
                let same_controller_status = property.property_type.same_controller_status();
                let rc_delta = EntityReferenceCounterSideEffect::atomic(same_controller_status, DeltaMode::Decrement);

                // Update InboundReferenceCounter of involved entity, based on previously calculated rc_delta
                Self::update_entity_rc(involved_entity_id, rc_delta);
                Some((involved_entity_id, rc_delta))
            } else {
                None
            };

            // Update entity property values
            <EntityById<T>>::mutate(entity_id, |entity| {
                entity.set_values(entity_values_updated);
            });

            // Trigger event
            Self::deposit_event(
                RawEvent::RemovedAtVectorIndex(
                    actor, entity_id, in_class_schema_property_id, index_in_property_vector,
                    nonce + T::Nonce::one(), involved_entity_and_side_effect
                )
            );

            Ok(())
        }

        /// Insert `SingleInputPropertyValue` at given `index_in_property_vector`
        /// into `PropertyValueVec` under `in_class_schema_property_id`
        pub fn insert_at_entity_property_vector(
            origin,
            actor: Actor<T>,
            entity_id: T::EntityId,
            in_class_schema_property_id: PropertyId,
            index_in_property_vector: VecMaxLength,
            value: InputValue<T>,
            nonce: T::Nonce
        ) -> dispatch::Result {

            // Retrieve Class, Entity and EntityAccessLevel for the actor, attemting to perform operation
            let (class, entity, access_level) = Self::ensure_class_entity_and_access_level(origin, entity_id, &actor)?;

            // Ensure Property under given PropertyId is unlocked from actor with given EntityAccessLevel
            // Retrieve corresponding Property by value
            let property = class.ensure_class_property_type_unlocked_from(
                in_class_schema_property_id,
                access_level,
            )?;

            // Ensure InputPropertyValue under given in_class_schema_property_id is Vector
            let property_value_vector =
                entity.ensure_property_value_is_vec(in_class_schema_property_id)?;

            // Ensure `VecInputPropertyValue` nonce is equal to the provided one.
            // Used to to avoid possible data races, when performing vector specific operations
            property_value_vector.ensure_nonce_equality(nonce)?;

            let entity_controller = entity.get_permissions_ref().get_controller();

            // Ensure property_value type is equal to the property_value_vector type and check all constraints
            property.ensure_property_value_can_be_inserted_at_property_vector(
                &value,
                &property_value_vector,
                index_in_property_vector,
                entity_controller,
            )?;

            let involved_entity = value.get_involved_entity();

            // Insert SingleInputPropertyValue at in_class_schema_property_id into property value vector
            // Get VecInputPropertyValue wrapped in InputPropertyValue
            let property_value_vector_updated = Self::insert_at_index_in_property_vector(
                property_value_vector.clone(), index_in_property_vector, value
            );

            let class_id = entity.get_class_id();

            // Compute old and new vec unique property value hash.
            // Ensure new property value hash with `unique` flag set is `unique` on `Class` level
            let vec_property_value_hashes = if property.unique {
                Some(
                    Self::ensure_vec_property_value_hashes(class_id, in_class_schema_property_id, &property_value_vector_updated, property_value_vector)?
                )
            } else {
                None
            };

            //
            // == MUTATION SAFE ==
            //

            if let Some((new_property_value_hash, old_property_value_hash)) = vec_property_value_hashes {
                // Add property value hash, that should be unique on `Class` level
                Self::add_unique_property_value_hash(class_id, in_class_schema_property_id, new_property_value_hash);

                // Remove property value hash, that should be unique on `Class` level
                Self::remove_unique_property_value_hash(class_id, in_class_schema_property_id, old_property_value_hash);
            }

            // Insert updated property value into entity_property_values mapping at in_class_schema_property_id.
            // Retrieve updated entity_property_values
            let entity_values_updated = Self::insert_at_in_class_schema_property_id(
                entity.get_values(), in_class_schema_property_id, property_value_vector_updated
            );

            // Increase reference counter of involved entity (if some)
            let involved_entity_and_side_effect = if let Some(entity_rc_to_increment) = involved_entity {
                let same_controller_status = property.property_type.same_controller_status();
                let rc_delta = EntityReferenceCounterSideEffect::atomic(same_controller_status, DeltaMode::Increment);

                // Update InboundReferenceCounter of involved entity, based on previously calculated ReferenceCounterSideEffect
                Self::update_entity_rc(entity_rc_to_increment, rc_delta);
                Some((entity_rc_to_increment, rc_delta))
            } else {
                None
            };

            // Update entity property values
            <EntityById<T>>::mutate(entity_id, |entity| {
                entity.set_values(entity_values_updated);
            });

            // Trigger event
            Self::deposit_event(
                RawEvent::InsertedAtVectorIndex(
                    actor, entity_id, in_class_schema_property_id, index_in_property_vector,
                    nonce + T::Nonce::one(), involved_entity_and_side_effect
                )
            );

            Ok(())
        }

        pub fn transaction(origin, actor: Actor<T>, operations: Vec<OperationType<T>>) -> dispatch::Result {

            // Ensure maximum number of operations during atomic batching limit not reached
            Self::ensure_number_of_operations_during_atomic_batching_limit_not_reached(&operations)?;

            //
            // == MUTATION SAFE ==
            //

            // This BTreeMap holds the T::EntityId of the entity created as a result of executing a `CreateEntity` `Operation`
            let mut entity_created_in_operation = BTreeMap::new();

            // Create raw origin
            let raw_origin = origin.into().map_err(|_| ERROR_ORIGIN_CANNOT_BE_MADE_INTO_RAW_ORIGIN)?;

            for (index, operation_type) in operations.into_iter().enumerate() {
                let origin = T::Origin::from(raw_origin.clone());
                let actor = actor.clone();
                match operation_type {
                    OperationType::CreateEntity(create_entity_operation) => {
                        Self::create_entity(origin, create_entity_operation.class_id, actor)?;

                        // entity id of newly created entity
                        let entity_id = Self::next_entity_id() - T::EntityId::one();
                        entity_created_in_operation.insert(index, entity_id);
                    },
                    OperationType::AddSchemaSupportToEntity(add_schema_support_to_entity_operation) => {
                        let entity_id = operations::parametrized_entity_to_entity_id(
                            &entity_created_in_operation, add_schema_support_to_entity_operation.entity_id
                        )?;
                        let schema_id = add_schema_support_to_entity_operation.schema_id;
                        let property_values = operations::parametrized_property_values_to_property_values(
                            &entity_created_in_operation, add_schema_support_to_entity_operation.parametrized_property_values
                        )?;
                        Self::add_schema_support_to_entity(origin, actor, entity_id, schema_id, property_values)?;
                    },
                    OperationType::UpdatePropertyValues(update_property_values_operation) => {
                        let entity_id = operations::parametrized_entity_to_entity_id(
                            &entity_created_in_operation, update_property_values_operation.entity_id
                        )?;
                        let property_values = operations::parametrized_property_values_to_property_values(
                            &entity_created_in_operation, update_property_values_operation.new_parametrized_property_values
                        )?;
                        Self::update_entity_property_values(origin, actor, entity_id, property_values)?;
                    },
                }
            }

            // Trigger event
            Self::deposit_event(RawEvent::TransactionCompleted(actor));

            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    /// Updates corresponding `Entity` `reference_counter` by `reference_counter_delta`.
    fn update_entity_rc(
        entity_id: T::EntityId,
        reference_counter_delta: EntityReferenceCounterSideEffect,
    ) {
        // Update both `total` and `same owner` number of inbound references for the Entity instance under given `entity_id`
        <EntityById<T>>::mutate(entity_id, |entity| {
            let entity_inbound_rc = entity.get_reference_counter_mut();
            entity_inbound_rc.total =
                (entity_inbound_rc.total as i32 + reference_counter_delta.total) as u32;
            entity_inbound_rc.same_owner =
                (entity_inbound_rc.same_owner as i32 + reference_counter_delta.same_owner) as u32;
        })
    }

    /// Add property value hash, that should be unique on `Class` level
    pub fn add_unique_property_value_hash(
        class_id: T::ClassId,
        property_id: PropertyId,
        hash: T::Hash,
    ) {
        <UniquePropertyValueHashes<T>>::insert((class_id, property_id), hash, ());
    }

    /// Remove property value hash, that should be unique on `Class` level
    pub fn remove_unique_property_value_hash(
        class_id: T::ClassId,
        property_id: PropertyId,
        hash: T::Hash,
    ) {
        <UniquePropertyValueHashes<T>>::remove((class_id, property_id), hash);
    }

    /// Add property value hashes, that should be unique on `Class` level
    pub fn add_unique_property_value_hashes(
        class_id: T::ClassId,
        unique_property_value_hashes: BTreeMap<PropertyId, T::Hash>,
    ) {
        unique_property_value_hashes
            .into_iter()
            .for_each(|(property_id, hash)| {
                Self::add_unique_property_value_hash(class_id, property_id, hash);
            });
    }

    /// Remove property value hashes, that should be unique on `Class` level
    pub fn remove_unique_property_value_hashes(
        class_id: T::ClassId,
        unique_property_value_hashes: BTreeMap<PropertyId, T::Hash>,
    ) {
        unique_property_value_hashes
            .into_iter()
            .for_each(|(property_id, hash)| {
                Self::remove_unique_property_value_hash(class_id, property_id, hash);
            });
    }

    /// Convert all provided `InputPropertyValue`'s into `StoredPropertyValue`'s
    pub fn make_output_property_values(
        input_property_values: BTreeMap<PropertyId, InputPropertyValue<T>>,
    ) -> BTreeMap<PropertyId, StoredPropertyValue<T>> {
        input_property_values
            .into_iter()
            .map(|(property_id, property_value)| (property_id, property_value.into()))
            .collect()
    }

    /// Update `entity_property_values` with `property_values`
    /// Returns updated `entity_property_values`
    fn make_updated_entity_property_values(
        schema: Schema,
        entity_property_values: BTreeMap<PropertyId, StoredPropertyValue<T>>,
        output_property_values: &BTreeMap<PropertyId, StoredPropertyValue<T>>,
    ) -> BTreeMap<PropertyId, StoredPropertyValue<T>> {
        // Concatenate existing `entity_property_values` with `property_values`, provided, when adding `Schema` support.
        let updated_entity_property_values: BTreeMap<PropertyId, StoredPropertyValue<T>> =
            entity_property_values
                .into_iter()
                .chain(output_property_values.to_owned().into_iter())
                .collect();

        // Write all missing non required `Schema` `property_values` as `InputPropertyValue::default()`
        let non_required_property_values: BTreeMap<PropertyId, StoredPropertyValue<T>> = schema
            .get_properties()
            .iter()
            .filter_map(|property_id| {
                if !updated_entity_property_values.contains_key(property_id) {
                    Some((*property_id, StoredPropertyValue::default()))
                } else {
                    None
                }
            })
            .collect();

        // Extend updated_entity_property_values with given Schema non_required_property_values
        updated_entity_property_values
            .into_iter()
            .chain(non_required_property_values.into_iter())
            .collect()
    }

    /// Calculate side effects for clear_property_vector operation, based on `property_value_vector` provided and its respective `property`.
    /// Returns calculated `ReferenceCounterSideEffects`
    pub fn make_side_effects_for_clear_property_vector_operation(
        property_value_vector: &VecStoredPropertyValue<T>,
        property: &Property<T>,
    ) -> Option<ReferenceCounterSideEffects<T>> {
        let entity_ids_to_decrease_rc = property_value_vector
            .get_vec_value_ref()
            .get_involved_entities();

        if let Some(entity_ids_to_decrease_rcs) = entity_ids_to_decrease_rc {
            // Calculate `ReferenceCounterSideEffects`, based on entity_ids involved, same_controller_status and chosen `DeltaMode`
            let same_controller_status = property.property_type.same_controller_status();
            let entities_inbound_rcs_delta = Self::perform_entities_inbound_rcs_delta_calculation(
                ReferenceCounterSideEffects::<T>::default(),
                entity_ids_to_decrease_rcs,
                same_controller_status,
                DeltaMode::Decrement,
            );

            if !entities_inbound_rcs_delta.is_empty() {
                Some(entities_inbound_rcs_delta)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Update `inbound_rcs_delta`, based on `involved_entity_ids`, `same_controller_status` provided and chosen `DeltaMode`
    /// Returns updated `inbound_rcs_delta`
    fn perform_entities_inbound_rcs_delta_calculation(
        mut inbound_rcs_delta: ReferenceCounterSideEffects<T>,
        involved_entity_ids: Vec<T::EntityId>,
        same_controller_status: bool,
        delta_mode: DeltaMode,
    ) -> ReferenceCounterSideEffects<T> {
        for involved_entity_id in involved_entity_ids {
            // If inbound_rcs_delta already contains entry for the given involved_entity_id, increment it
            // with atomic EntityReferenceCounterSideEffect instance, based on same_owner flag provided and DeltaMode,
            // otherwise create new atomic EntityReferenceCounterSideEffect instance
            if let Some(inbound_rc_delta) = inbound_rcs_delta.get_mut(&involved_entity_id) {
                *inbound_rc_delta +=
                    EntityReferenceCounterSideEffect::atomic(same_controller_status, delta_mode);
            } else {
                inbound_rcs_delta.insert(
                    involved_entity_id,
                    EntityReferenceCounterSideEffect::atomic(same_controller_status, delta_mode),
                );
            }
        }
        inbound_rcs_delta
    }

    /// Filter references, pointing to the same `Entity`
    fn filter_references_to_the_same_entity(
        current_entity_id: T::EntityId,
        involved_entity_ids: Vec<T::EntityId>,
    ) -> Vec<T::EntityId> {
        involved_entity_ids
            .into_iter()
            .filter(|involved_entity_id| current_entity_id != *involved_entity_id)
            .collect()
    }

    /// Calculate `ReferenceCounterSideEffects`, based on `values_for_existing_properties` provided and chosen `DeltaMode`
    /// Returns calculated `ReferenceCounterSideEffects`
    fn calculate_entities_inbound_rcs_delta(
        current_entity_id: T::EntityId,
        values_for_existing_properties: StoredValuesForExistingProperties<T>,
        delta_mode: DeltaMode,
    ) -> Option<ReferenceCounterSideEffects<T>> {
        let entities_inbound_rcs_delta = values_for_existing_properties
            .values()
            .map(|value_for_existing_property| value_for_existing_property.unzip())
            .filter_map(|(property, value)| {
                let involved_entity_ids =
                    value.get_involved_entities().map(|involved_entity_ids| {
                        Self::filter_references_to_the_same_entity(
                            current_entity_id,
                            involved_entity_ids,
                        )
                    });
                match involved_entity_ids {
                    Some(involved_entity_ids) if !involved_entity_ids.is_empty() => Some((
                        involved_entity_ids,
                        property.property_type.same_controller_status(),
                    )),
                    _ => None,
                }
            })
            // Aggeregate all sideffects on a single entity together into one side effect map
            .fold(
                ReferenceCounterSideEffects::default(),
                |inbound_rcs_delta, (involved_entity_ids, same_controller_status)| {
                    Self::perform_entities_inbound_rcs_delta_calculation(
                        inbound_rcs_delta,
                        involved_entity_ids,
                        same_controller_status,
                        delta_mode,
                    )
                },
            );

        if !entities_inbound_rcs_delta.is_empty() {
            Some(entities_inbound_rcs_delta)
        } else {
            None
        }
    }

    /// Compute `ReferenceCounterSideEffects`, based on `InputPropertyValue` `Reference`'s involved into update process.
    /// Returns updated `ReferenceCounterSideEffects`
    pub fn get_updated_inbound_rcs_delta(
        current_entity_id: T::EntityId,
        class_properties: Vec<Property<T>>,
        entity_property_values: BTreeMap<PropertyId, StoredPropertyValue<T>>,
        new_output_property_values: BTreeMap<PropertyId, StoredPropertyValue<T>>,
    ) -> Result<Option<ReferenceCounterSideEffects<T>>, &'static str> {
        // Filter entity_property_values to get only those, which will be substituted with new_property_values
        let entity_property_values_to_update: BTreeMap<PropertyId, StoredPropertyValue<T>> =
            entity_property_values
                .into_iter()
                .filter(|(entity_id, _)| new_output_property_values.contains_key(entity_id))
                .collect();

        // Calculate entities reference counter side effects for update operation

        let stored_values_for_entity_property_values_to_update =
            match StoredValuesForExistingProperties::from(
                &class_properties,
                &entity_property_values_to_update,
            ) {
                Ok(stored_values_for_entity_property_values_to_update) => {
                    stored_values_for_entity_property_values_to_update
                }
                Err(e) => {
                    debug_assert!(false, "Should not fail! {:?}", e);
                    return Err(e);
                }
            };

        // Calculate entities inbound reference counter delta with Decrement DeltaMode for entity_property_values_to_update,
        // as involved InputPropertyValue References will be substituted with new ones
        let decremental_reference_counter_side_effects = Self::calculate_entities_inbound_rcs_delta(
            current_entity_id,
            stored_values_for_entity_property_values_to_update,
            DeltaMode::Decrement,
        );

        // Calculate entities inbound reference counter delta with Increment DeltaMode for new_property_values,
        // as involved InputPropertyValue References will substitute the old ones
        let incremental_reference_counter_side_effects = Self::calculate_entities_inbound_rcs_delta(
            current_entity_id,
            StoredValuesForExistingProperties::from(
                &class_properties,
                &new_output_property_values,
            )?,
            DeltaMode::Increment,
        );

        // Add up both net decremental_reference_counter_side_effects and incremental_reference_counter_side_effects
        // to get one net sideffect per entity.
        Ok(Self::calculate_updated_inbound_rcs_delta(
            decremental_reference_counter_side_effects,
            incremental_reference_counter_side_effects,
        ))
    }

    /// Add up both net first_reference_counter_side_effects and second_reference_counter_side_effects (if some)
    /// to get one net sideffect per entity.
    /// Returns updated `ReferenceCounterSideEffects`
    pub fn calculate_updated_inbound_rcs_delta(
        first_reference_counter_side_effects: Option<ReferenceCounterSideEffects<T>>,
        second_reference_counter_side_effects: Option<ReferenceCounterSideEffects<T>>,
    ) -> Option<ReferenceCounterSideEffects<T>> {
        match (
            first_reference_counter_side_effects,
            second_reference_counter_side_effects,
        ) {
            (
                Some(first_reference_counter_side_effects),
                Some(second_reference_counter_side_effects),
            ) => {
                let reference_counter_side_effects = first_reference_counter_side_effects
                    .update(second_reference_counter_side_effects);
                Some(reference_counter_side_effects)
            }
            (Some(first_reference_counter_side_effects), _) => {
                Some(first_reference_counter_side_effects)
            }
            (_, Some(second_reference_counter_side_effects)) => {
                Some(second_reference_counter_side_effects)
            }
            _ => None,
        }
    }

    /// Used to update `class_permissions` with parameters provided.
    /// Returns updated `class_permissions` if update performed
    pub fn make_updated_class_permissions(
        class_permissions: ClassPermissions<T>,
        updated_any_member: Option<bool>,
        updated_entity_creation_blocked: Option<bool>,
        updated_all_entity_property_values_locked: Option<bool>,
        updated_maintainers: Option<BTreeSet<T::CuratorGroupId>>,
    ) -> Option<ClassPermissions<T>> {
        // Used to check if update performed
        let mut updated_class_permissions = class_permissions.clone();

        if let Some(updated_any_member) = updated_any_member {
            updated_class_permissions.set_any_member_status(updated_any_member);
        }

        if let Some(updated_entity_creation_blocked) = updated_entity_creation_blocked {
            updated_class_permissions.set_entity_creation_blocked(updated_entity_creation_blocked);
        }

        if let Some(updated_all_entity_property_values_locked) =
            updated_all_entity_property_values_locked
        {
            updated_class_permissions
                .set_all_entity_property_values_locked(updated_all_entity_property_values_locked);
        }

        if let Some(updated_maintainers) = updated_maintainers {
            updated_class_permissions.set_maintainers(updated_maintainers);
        }

        if updated_class_permissions != class_permissions {
            Some(updated_class_permissions)
        } else {
            None
        }
    }

    /// Used to update `entity_permissions` with parameters provided.
    /// Returns updated `entity_permissions` if update performed
    pub fn make_updated_entity_permissions(
        entity_permissions: EntityPermissions<T>,
        updated_frozen: Option<bool>,
        updated_referenceable: Option<bool>,
    ) -> Option<EntityPermissions<T>> {
        // Used to check if update performed
        let mut updated_entity_permissions = entity_permissions.clone();

        if let Some(updated_frozen) = updated_frozen {
            updated_entity_permissions.set_frozen(updated_frozen);
        }

        if let Some(updated_referenceable) = updated_referenceable {
            updated_entity_permissions.set_referencable(updated_referenceable);
        }

        if updated_entity_permissions != entity_permissions {
            Some(updated_entity_permissions)
        } else {
            None
        }
    }

    /// Ensure property value hash with `unique` flag set is `unique` on `Class` level
    pub fn ensure_property_value_hash_unique_option_satisfied(
        class_id: T::ClassId,
        property_id: PropertyId,
        unique_property_value_hash: &T::Hash,
    ) -> Result<(), &'static str> {
        ensure!(
            !<UniquePropertyValueHashes<T>>::exists(
                (class_id, property_id),
                unique_property_value_hash
            ),
            ERROR_PROPERTY_VALUE_SHOULD_BE_UNIQUE
        );
        Ok(())
    }

    /// Ensure all property value hashes with `unique` flag set are `unique` on `Class` level
    pub fn ensure_property_value_hashes_unique_option_satisfied(
        class_id: T::ClassId,
        unique_property_value_hashes: &BTreeMap<PropertyId, T::Hash>,
    ) -> Result<(), &'static str> {
        for (&property_id, unique_property_value_hash) in unique_property_value_hashes {
            Self::ensure_property_value_hash_unique_option_satisfied(
                class_id,
                property_id,
                unique_property_value_hash,
            )?;
        }
        Ok(())
    }

    /// Compute old and new vec unique property value hash.
    /// Ensure new property value hash with `unique` flag set is `unique` on `Class` level
    pub fn ensure_vec_property_value_hashes(
        class_id: T::ClassId,
        in_class_schema_property_id: PropertyId,
        property_value_vector_updated: &StoredPropertyValue<T>,
        property_value_vector: VecStoredPropertyValue<T>,
    ) -> Result<(T::Hash, T::Hash), &'static str> {
        // Compute new hash from unique property value and its respective property id
        let new_property_value_hash =
            property_value_vector_updated.compute_unique_hash(in_class_schema_property_id);

        // Ensure `Property` with `unique` flag set is `unique` on `Class` level
        Self::ensure_property_value_hash_unique_option_satisfied(
            class_id,
            in_class_schema_property_id,
            &new_property_value_hash,
        )?;

        // Compute old hash from the old unique property value and its respective property id
        let old_property_value_hash =
            property_value_vector.compute_unique_hash(in_class_schema_property_id);

        Ok((new_property_value_hash, old_property_value_hash))
    }

    /// Compute new unique property value hashes.
    /// Ensure new property value hashes with `unique` flag set are `unique` on `Class` level
    pub fn ensure_new_property_values_respect_uniquness(
        class_id: T::ClassId,
        new_output_values_for_existing_properties: StoredValuesForExistingProperties<T>,
    ) -> Result<BTreeMap<PropertyId, T::Hash>, &'static str> {
        let new_unique_property_value_hashes =
            new_output_values_for_existing_properties.compute_unique_hashes();

        // Ensure all provided Properties with unique flag set are unique on Class level
        Self::ensure_property_value_hashes_unique_option_satisfied(
            class_id,
            &new_unique_property_value_hashes,
        )?;

        Ok(new_unique_property_value_hashes)
    }

    /// Returns the stored `Class` if exist, error otherwise.
    fn ensure_class_exists(class_id: T::ClassId) -> Result<Class<T>, &'static str> {
        ensure!(<ClassById<T>>::exists(class_id), ERROR_CLASS_NOT_FOUND);
        Ok(Self::class_by_id(class_id))
    }

    /// Returns `Class` and `Entity` under given id, if exists, and `EntityAccessLevel` corresponding to `origin`, if permitted
    fn ensure_class_entity_and_access_level(
        origin: T::Origin,
        entity_id: T::EntityId,
        actor: &Actor<T>,
    ) -> Result<(Class<T>, Entity<T>, EntityAccessLevel), &'static str> {
        let account_id = ensure_signed(origin)?;

        // Ensure Entity under given id exists, retrieve corresponding one
        let entity = Self::ensure_known_entity_id(entity_id)?;

        // Retrieve corresponding Class
        let class = Self::class_by_id(entity.get_class_id());

        // Derive EntityAccessLevel for the actor, attempting to act.
        let access_level = EntityAccessLevel::derive(
            &account_id,
            entity.get_permissions_ref(),
            class.get_permissions_ref(),
            actor,
        )?;

        Ok((class, entity, access_level))
    }

    /// Ensure `Entity` under given `entity_id` exists, retrieve corresponding `Entity` & `Class`
    pub fn ensure_known_entity_and_class(
        entity_id: T::EntityId,
    ) -> Result<(Entity<T>, Class<T>), &'static str> {
        // Ensure Entity under given id exists, retrieve corresponding one
        let entity = Self::ensure_known_entity_id(entity_id)?;

        let class = ClassById::get(entity.get_class_id());
        Ok((entity, class))
    }

    /// Filter `provided values_for_existing_properties`, leaving only `Reference`'s with `SameOwner` flag set
    /// Returns the set of corresponding property ids
    pub fn get_property_id_references_with_same_owner_flag_set(
        values_for_existing_properties: StoredValuesForExistingProperties<T>,
    ) -> BTreeSet<PropertyId> {
        values_for_existing_properties
            // Iterate over the PropertyId's
            .keys()
            // Filter provided values_for_existing_properties, leaving only `Reference`'s with `SameOwner` flag set
            .filter(|property_id| {
                if let Some(value_for_existing_property) =
                    values_for_existing_properties.get(property_id)
                {
                    value_for_existing_property
                        .get_property()
                        .property_type
                        .same_controller_status()
                } else {
                    false
                }
            })
            .copied()
            .collect()
    }

    // Ensure all ids of provided `new_property_value_references_with_same_owner_flag_set`
    // corresponding to property ids of respective Class Property references with same owner flag set
    pub fn ensure_only_reference_ids_with_same_owner_flag_set_provided(
        entity_property_id_references_with_same_owner_flag_set: &BTreeSet<PropertyId>,
        new_property_value_references_with_same_owner_flag_set: &BTreeMap<
            PropertyId,
            InputPropertyValue<T>,
        >,
    ) -> dispatch::Result {
        let new_property_value_id_references_with_same_owner_flag_set: BTreeSet<PropertyId> =
            new_property_value_references_with_same_owner_flag_set
                .keys()
                .copied()
                .collect();

        ensure!(
            new_property_value_id_references_with_same_owner_flag_set
                .is_subset(entity_property_id_references_with_same_owner_flag_set),
            ERROR_ALL_PROVIDED_PROPERTY_VALUE_IDS_MUST_BE_REFERENCES_WITH_SAME_OWNER_FLAG_SET
        );
        Ok(())
    }

    /// Ensure all provided `new_property_value_references_with_same_owner_flag_set` are valid
    fn ensure_are_valid_references_with_same_owner_flag_set(
        new_property_value_references_with_same_owner_flag_set: InputValuesForExistingProperties<T>,
        new_controller: &EntityController<T>,
    ) -> dispatch::Result {
        for updated_value_for_existing_property in
            new_property_value_references_with_same_owner_flag_set.values()
        {
            let (property, value) = updated_value_for_existing_property.unzip();

            // Perform all required checks to ensure provided property values are valid references
            property.ensure_property_value_is_valid_reference(value, new_controller)?;
        }
        Ok(())
    }

    /// Used to update entity_property_values with parameters provided.
    /// Returns updated `entity_property_values`, if update performed
    pub fn make_updated_property_value_references_with_same_owner_flag_set(
        unused_property_id_references_with_same_owner_flag_set: BTreeSet<PropertyId>,
        entity_property_values: &BTreeMap<PropertyId, StoredPropertyValue<T>>,
        new_property_value_references_with_same_owner_flag_set: &BTreeMap<
            PropertyId,
            StoredPropertyValue<T>,
        >,
    ) -> Option<BTreeMap<PropertyId, StoredPropertyValue<T>>> {
        // Used to check if update performed
        let mut entity_property_values_updated = entity_property_values.clone();

        for (property_id, new_property_value_reference_with_same_owner_flag_set) in
            new_property_value_references_with_same_owner_flag_set
        {
            // Update entity_property_values map at property_id with new_property_value_reference_with_same_owner_flag_set
            entity_property_values_updated.insert(
                *property_id,
                new_property_value_reference_with_same_owner_flag_set.to_owned(),
            );
        }

        // Throw away old non required property value references with same owner flag set
        // and replace them with Default ones
        for unused_property_id_reference_with_same_owner_flag_set in
            unused_property_id_references_with_same_owner_flag_set
        {
            entity_property_values_updated.insert(
                unused_property_id_reference_with_same_owner_flag_set,
                StoredPropertyValue::default(),
            );
        }

        if *entity_property_values != entity_property_values_updated {
            Some(entity_property_values_updated)
        } else {
            None
        }
    }

    // Update InboundReferenceCounter, based on previously calculated entities_inbound_rcs_delta, for each Entity involved
    pub fn update_entities_rcs(
        entities_inbound_rcs_delta: &Option<ReferenceCounterSideEffects<T>>,
    ) {
        if let Some(entities_inbound_rcs_delta) = entities_inbound_rcs_delta {
            entities_inbound_rcs_delta.update_entities_rcs();
        }
    }

    /// Retrieve `property_ids`, that are not in `property_values`
    pub fn compute_unused_property_ids(
        property_values: &BTreeMap<PropertyId, InputPropertyValue<T>>,
        property_ids: &BTreeSet<PropertyId>,
    ) -> BTreeSet<PropertyId> {
        let property_value_indices: BTreeSet<PropertyId> =
            property_values.keys().cloned().collect();

        property_ids
            .difference(&property_value_indices)
            .copied()
            .collect()
    }

    /// Used to compute old unique hashes, that should be substituted with new ones.
    pub fn compute_old_unique_hashes(
        new_output_property_values: &BTreeMap<PropertyId, StoredPropertyValue<T>>,
        entity_values: &BTreeMap<PropertyId, StoredPropertyValue<T>>,
    ) -> BTreeMap<PropertyId, T::Hash> {
        entity_values
            .iter()
            .filter(|(property_id, _)| new_output_property_values.contains_key(property_id))
            .map(|(&property_id, property_value)| {
                (property_id, property_value.compute_unique_hash(property_id))
            })
            .collect()
    }

    /// Perform checks to ensure all required `property_values` under provided `unused_schema_property_ids` provided
    pub fn ensure_all_required_properties_provided(
        class_properties: &[Property<T>],
        unused_schema_property_ids: &BTreeSet<PropertyId>,
    ) -> dispatch::Result {
        for &unused_schema_property_id in unused_schema_property_ids {
            let class_property = &class_properties
                .get(unused_schema_property_id as usize)
                .ok_or(ERROR_CLASS_PROP_NOT_FOUND)?;

            // All required property values should be provided
            ensure!(!class_property.required, ERROR_MISSING_REQUIRED_PROP);
        }
        Ok(())
    }

    /// Validate all values, provided in `values_for_existing_properties`, against the type of its `Property`
    /// and check any additional constraints
    pub fn ensure_property_values_are_valid(
        entity_controller: &EntityController<T>,
        values_for_existing_properties: &InputValuesForExistingProperties<T>,
    ) -> dispatch::Result {
        for value_for_existing_property in values_for_existing_properties.values() {
            let (property, value) = value_for_existing_property.unzip();

            // Validate new InputPropertyValue against the type of this Property and check any additional constraints
            property.ensure_property_value_to_update_is_valid(value, entity_controller)?;
        }

        Ok(())
    }

    /// Ensure all provided `new_property_values` are already exist in `entity_property_values` map
    pub fn ensure_all_property_values_are_already_added(
        entity_property_values: &BTreeMap<PropertyId, StoredPropertyValue<T>>,
        new_property_values: &BTreeMap<PropertyId, InputPropertyValue<T>>,
    ) -> dispatch::Result {
        ensure!(
            new_property_values
                .keys()
                .all(|key| entity_property_values.contains_key(key)),
            ERROR_UNKNOWN_ENTITY_PROP_ID
        );
        Ok(())
    }

    /// Ensure `new_values_for_existing_properties` are accessible for actor with given `access_level`
    pub fn ensure_all_property_values_are_unlocked_from(
        new_values_for_existing_properties: &InputValuesForExistingProperties<T>,
        access_level: EntityAccessLevel,
    ) -> dispatch::Result {
        for value_for_new_property in new_values_for_existing_properties.values() {
            // Ensure Property is unlocked from Actor with given EntityAccessLevel
            value_for_new_property
                .get_property()
                .ensure_unlocked_from(access_level)?;
        }
        Ok(())
    }

    /// Filter `new_property_values` identical to `entity_property_values`.
    /// Return only `new_property_values`, that are not in `entity_property_values`
    pub fn try_filter_identical_property_values(
        entity_property_values: &BTreeMap<PropertyId, StoredPropertyValue<T>>,
        new_property_values: BTreeMap<PropertyId, InputPropertyValue<T>>,
    ) -> BTreeMap<PropertyId, InputPropertyValue<T>> {
        new_property_values
            .into_iter()
            .filter(|(id, new_property_value)| {
                if let Some(entity_property_value) = entity_property_values.get(id) {
                    StoredPropertyValue::<T>::from(new_property_value.to_owned())
                        != *entity_property_value
                } else {
                    true
                }
            })
            .collect()
    }

    /// Update existing `entity_property_values` with `new_property_values`.
    /// if update performed, returns updated entity property values
    pub fn make_updated_property_values(
        entity_property_values: &BTreeMap<PropertyId, StoredPropertyValue<T>>,
        new_output_property_values: &BTreeMap<PropertyId, StoredPropertyValue<T>>,
    ) -> Option<BTreeMap<PropertyId, StoredPropertyValue<T>>> {
        // Used to check if updated performed
        let mut entity_property_values_updated = entity_property_values.to_owned();

        new_output_property_values
            .iter()
            .for_each(|(id, new_property_value)| {
                if let Some(entity_property_value) = entity_property_values_updated.get_mut(&id) {
                    entity_property_value.update(new_property_value.to_owned());
                }
            });

        if entity_property_values_updated != *entity_property_values {
            Some(entity_property_values_updated)
        } else {
            None
        }
    }

    /// Insert `InputValue` into `VecStoredPropertyValue` at `index_in_property_vector`.
    /// Returns `VecStoredPropertyValue` wrapped in `StoredPropertyValue`
    pub fn insert_at_index_in_property_vector(
        mut property_value_vector: VecStoredPropertyValue<T>,
        index_in_property_vector: VecMaxLength,
        value: InputValue<T>,
    ) -> StoredPropertyValue<T> {
        property_value_vector.insert_at(index_in_property_vector, value.into());
        StoredPropertyValue::Vector(property_value_vector)
    }

    /// Remove `InputValue` at `index_in_property_vector` in `VecInputPropertyValue`.
    /// Returns `VecInputPropertyValue` wrapped in `InputPropertyValue`
    pub fn remove_at_index_in_property_vector(
        mut property_value_vector: VecStoredPropertyValue<T>,
        index_in_property_vector: VecMaxLength,
    ) -> StoredPropertyValue<T> {
        property_value_vector.remove_at(index_in_property_vector);
        StoredPropertyValue::Vector(property_value_vector)
    }

    /// Clear `VecStoredPropertyValue`.
    /// Returns empty `VecStoredPropertyValue` wrapped in `StoredPropertyValue`
    pub fn clear_property_vector(
        mut property_value_vector: VecStoredPropertyValue<T>,
    ) -> StoredPropertyValue<T> {
        property_value_vector.clear();
        StoredPropertyValue::Vector(property_value_vector)
    }

    /// Insert `InputPropertyValue` into `entity_property_values` mapping at `in_class_schema_property_id`.
    /// Returns updated `entity_property_values`
    pub fn insert_at_in_class_schema_property_id(
        mut entity_property_values: BTreeMap<PropertyId, StoredPropertyValue<T>>,
        in_class_schema_property_id: PropertyId,
        property_value: StoredPropertyValue<T>,
    ) -> BTreeMap<PropertyId, StoredPropertyValue<T>> {
        entity_property_values.insert(in_class_schema_property_id, property_value);
        entity_property_values
    }

    /// Ensure `Class` under given id exists, return corresponding one
    pub fn ensure_known_class_id(class_id: T::ClassId) -> Result<Class<T>, &'static str> {
        ensure!(<ClassById<T>>::exists(class_id), ERROR_CLASS_NOT_FOUND);
        Ok(Self::class_by_id(class_id))
    }

    /// Ensure `Entity` under given id exists, return corresponding one
    pub fn ensure_known_entity_id(entity_id: T::EntityId) -> Result<Entity<T>, &'static str> {
        ensure!(<EntityById<T>>::exists(entity_id), ERROR_ENTITY_NOT_FOUND);
        Ok(Self::entity_by_id(entity_id))
    }

    /// Ensure `CuratorGroup` under given id exists
    pub fn ensure_curator_group_under_given_id_exists(
        curator_group_id: &T::CuratorGroupId,
    ) -> dispatch::Result {
        ensure!(
            <CuratorGroupById<T>>::exists(curator_group_id),
            ERROR_CURATOR_GROUP_DOES_NOT_EXIST
        );
        Ok(())
    }

    /// Ensure `CuratorGroup` under given id exists, return corresponding one
    pub fn ensure_curator_group_exists(
        curator_group_id: &T::CuratorGroupId,
    ) -> Result<CuratorGroup<T>, &'static str> {
        Self::ensure_curator_group_under_given_id_exists(curator_group_id)?;
        Ok(Self::curator_group_by_id(curator_group_id))
    }

    /// Ensure `MaxNumberOfMaintainersPerClass` constraint satisfied
    pub fn ensure_maintainers_limit_not_reached(
        curator_groups: &BTreeSet<T::CuratorGroupId>,
    ) -> dispatch::Result {
        ensure!(
            curator_groups.len() < T::MaxNumberOfMaintainersPerClass::get() as usize,
            ERROR_NUMBER_OF_MAINTAINERS_PER_CLASS_LIMIT_REACHED
        );
        Ok(())
    }

    /// Ensure all `CuratorGroup`'s under given ids exist
    pub fn ensure_curator_groups_exist(
        curator_groups: &BTreeSet<T::CuratorGroupId>,
    ) -> dispatch::Result {
        for curator_group in curator_groups {
            // Ensure CuratorGroup under given id exists
            Self::ensure_curator_group_exists(curator_group)?;
        }
        Ok(())
    }

    /// Perform security checks to ensure provided `class_maintainers` are valid
    pub fn ensure_class_maintainers_are_valid(
        class_maintainers: &BTreeSet<T::CuratorGroupId>,
    ) -> dispatch::Result {
        // Ensure max number of maintainers per Class constraint satisfied
        ensure!(
            class_maintainers.len() <= T::MaxNumberOfMaintainersPerClass::get() as usize,
            ERROR_NUMBER_OF_MAINTAINERS_PER_CLASS_LIMIT_REACHED
        );

        // Ensure all curator groups provided are already exist in runtime
        Self::ensure_curator_groups_exist(class_maintainers)?;
        Ok(())
    }

    /// Ensure new `Schema` is not empty
    pub fn ensure_non_empty_schema(
        existing_properties: &BTreeSet<PropertyId>,
        new_properties: &[Property<T>],
    ) -> dispatch::Result {
        // Schema is empty if both existing_properties and new_properties are empty
        let non_empty_schema = !existing_properties.is_empty() || !new_properties.is_empty();
        ensure!(non_empty_schema, ERROR_NO_PROPS_IN_CLASS_SCHEMA);
        Ok(())
    }

    /// Ensure `ClassNameLengthConstraint` conditions satisfied
    pub fn ensure_class_name_is_valid(text: &[u8]) -> dispatch::Result {
        T::ClassNameLengthConstraint::get().ensure_valid(
            text.len(),
            ERROR_CLASS_NAME_TOO_SHORT,
            ERROR_CLASS_NAME_TOO_LONG,
        )
    }

    /// Ensure `ClassDescriptionLengthConstraint` conditions satisfied
    pub fn ensure_class_description_is_valid(text: &[u8]) -> dispatch::Result {
        T::ClassDescriptionLengthConstraint::get().ensure_valid(
            text.len(),
            ERROR_CLASS_DESCRIPTION_TOO_SHORT,
            ERROR_CLASS_DESCRIPTION_TOO_LONG,
        )
    }

    /// Ensure `MaxNumberOfClasses` constraint satisfied
    pub fn ensure_class_limit_not_reached() -> dispatch::Result {
        ensure!(
            (<ClassById<T>>::enumerate().count() as MaxNumber) < T::MaxNumberOfClasses::get(),
            ERROR_CLASS_LIMIT_REACHED
        );
        Ok(())
    }

    /// Ensure `MaxNumberOfEntitiesPerClass` constraint satisfied
    pub fn ensure_valid_number_of_entities_per_class(
        maximum_entities_count: T::EntityId,
    ) -> dispatch::Result {
        ensure!(
            maximum_entities_count <= T::MaxNumberOfEntitiesPerClass::get(),
            ERROR_ENTITIES_NUMBER_PER_CLASS_CONSTRAINT_VIOLATED
        );
        Ok(())
    }

    /// Ensure `IndividualEntitiesCreationLimit` constraint satisfied
    pub fn ensure_valid_number_of_class_entities_per_actor_constraint(
        number_of_class_entities_per_actor: T::EntityId,
    ) -> dispatch::Result {
        ensure!(
            number_of_class_entities_per_actor <= T::IndividualEntitiesCreationLimit::get(),
            ERROR_NUMBER_OF_CLASS_ENTITIES_PER_ACTOR_CONSTRAINT_VIOLATED
        );
        Ok(())
    }

    /// Ensure all entities creation limits, defined for a given `Class`, are valid
    pub fn ensure_entities_creation_limits_are_valid(
        maximum_entities_count: T::EntityId,
        default_entity_creation_voucher_upper_bound: T::EntityId,
    ) -> dispatch::Result {
        // Ensure `per_controller_entities_creation_limit` does not exceed
        ensure!(
            default_entity_creation_voucher_upper_bound < maximum_entities_count,
            ERROR_PER_CONTROLLER_ENTITIES_CREATION_LIMIT_EXCEEDS_OVERALL_LIMIT
        );

        // Ensure maximum_entities_count does not exceed MaxNumberOfEntitiesPerClass limit
        Self::ensure_valid_number_of_entities_per_class(maximum_entities_count)?;

        // Ensure default_entity_creation_voucher_upper_bound constraint does not exceed IndividualEntitiesCreationLimit
        Self::ensure_valid_number_of_class_entities_per_actor_constraint(
            default_entity_creation_voucher_upper_bound,
        )
    }

    /// Ensure maximum number of operations during atomic batching constraint satisfied
    pub fn ensure_number_of_operations_during_atomic_batching_limit_not_reached(
        operations: &[OperationType<T>],
    ) -> dispatch::Result {
        ensure!(
            operations.len() <= T::MaxNumberOfOperationsDuringAtomicBatching::get() as usize,
            ERROR_MAX_NUMBER_OF_OPERATIONS_DURING_ATOMIC_BATCHING_LIMIT_REACHED
        );
        Ok(())
    }

    /// Complete all checks to ensure each `Property` is valid
    pub fn ensure_all_properties_are_valid(new_properties: &[Property<T>]) -> dispatch::Result {
        for new_property in new_properties.iter() {
            // Ensure PropertyNameLengthConstraint satisfied
            new_property.ensure_name_is_valid()?;

            // Ensure PropertyDescriptionLengthConstraint satisfied
            new_property.ensure_description_is_valid()?;

            // Ensure Type specific constraints satisfied
            new_property.ensure_property_type_size_is_valid()?;

            // Ensure refers to existing class_id, if If Property Type is Reference,
            new_property.ensure_property_type_reference_is_valid()?;
        }
        Ok(())
    }

    /// Ensure all `Property` names are  unique within `Class`
    pub fn ensure_all_property_names_are_unique(
        class_properties: &[Property<T>],
        new_properties: &[Property<T>],
    ) -> dispatch::Result {
        // Used to ensure all property names are unique within class
        let mut unique_prop_names = BTreeSet::new();

        for property in class_properties.iter() {
            unique_prop_names.insert(property.name.to_owned());
        }

        for new_property in new_properties {
            // Ensure name of a new property is unique within its class.
            ensure!(
                !unique_prop_names.contains(&new_property.name),
                ERROR_PROP_NAME_NOT_UNIQUE_IN_A_CLASS
            );

            unique_prop_names.insert(new_property.name.to_owned());
        }

        Ok(())
    }

    /// Ensure provided indices of `existing_properties`  are valid indices of `Class` properties
    pub fn ensure_schema_properties_are_valid_indices(
        existing_properties: &BTreeSet<PropertyId>,
        class_properties: &[Property<T>],
    ) -> dispatch::Result {
        let has_unknown_properties = existing_properties
            .iter()
            .any(|&prop_id| prop_id >= class_properties.len() as PropertyId);
        ensure!(
            !has_unknown_properties,
            ERROR_CLASS_SCHEMA_REFERS_UNKNOWN_PROP_INDEX
        );
        Ok(())
    }

    /// Create new `Schema` from existing and new property ids
    pub fn create_class_schema(
        existing_properties: BTreeSet<PropertyId>,
        class_properties: &[Property<T>],
        new_properties: &[Property<T>],
    ) -> Schema {
        // Calcualate new property ids
        let properties = new_properties
            .iter()
            .enumerate()
            .map(|(i, _)| (class_properties.len() + i) as PropertyId)
            // Concatenate them with existing ones
            .chain(existing_properties.into_iter())
            .collect();

        Schema::new(properties)
    }

    /// Update existing `Class` properties with new ones provided, return updated ones
    pub fn make_updated_class_properties(
        class_properties: Vec<Property<T>>,
        new_properties: Vec<Property<T>>,
    ) -> Vec<Property<T>> {
        class_properties
            .into_iter()
            .chain(new_properties.into_iter())
            .collect()
    }
}

decl_event!(
    pub enum Event<T>
    where
        CuratorGroupId = <T as ActorAuthenticator>::CuratorGroupId,
        CuratorId = <T as ActorAuthenticator>::CuratorId,
        ClassId = <T as Trait>::ClassId,
        EntityId = <T as Trait>::EntityId,
        EntityController = EntityController<T>,
        EntityCreationVoucher = EntityCreationVoucher<T>,
        Status = bool,
        Actor = Actor<T>,
        Nonce = <T as Trait>::Nonce,
        SideEffects = Option<ReferenceCounterSideEffects<T>>,
        SideEffect = Option<(<T as Trait>::EntityId, EntityReferenceCounterSideEffect)>,
    {
        CuratorGroupAdded(CuratorGroupId),
        CuratorGroupRemoved(CuratorGroupId),
        CuratorGroupStatusSet(CuratorGroupId, Status),
        CuratorAdded(CuratorGroupId, CuratorId),
        CuratorRemoved(CuratorGroupId, CuratorId),
        MaintainerAdded(ClassId, CuratorGroupId),
        MaintainerRemoved(ClassId, CuratorGroupId),
        EntityCreationVoucherUpdated(EntityController, EntityCreationVoucher),
        EntityCreationVoucherCreated(EntityController, EntityCreationVoucher),
        ClassCreated(ClassId),
        ClassPermissionsUpdated(ClassId),
        ClassSchemaAdded(ClassId, SchemaId),
        ClassSchemaStatusUpdated(ClassId, SchemaId, Status),
        EntityPermissionsUpdated(EntityId),
        EntityCreated(Actor, EntityId),
        EntityRemoved(Actor, EntityId),
        EntitySchemaSupportAdded(Actor, EntityId, SchemaId, SideEffects),
        EntityPropertyValuesUpdated(Actor, EntityId, SideEffects),
        VectorCleared(Actor, EntityId, PropertyId, SideEffects),
        RemovedAtVectorIndex(Actor, EntityId, PropertyId, VecMaxLength, Nonce, SideEffect),
        InsertedAtVectorIndex(Actor, EntityId, PropertyId, VecMaxLength, Nonce, SideEffect),
        EntityOwnershipTransfered(EntityId, EntityController, SideEffects),
        TransactionCompleted(Actor),
    }
);
