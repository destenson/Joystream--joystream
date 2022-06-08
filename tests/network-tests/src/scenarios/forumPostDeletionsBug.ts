import leadOpening from '../flows/working-groups/leadOpening'
import multiplePostDeletionsBug from '../flows/forum/multiplePostDeletionsBug'
import { scenario } from '../Scenario'

scenario('Forum post deletions bug', async ({ job }) => {
  const sudoHireLead = job('hiring working group leads', leadOpening())
  job('forum post deletions bug', multiplePostDeletionsBug).requires(sudoHireLead)
})
