using Devon4Net.Domain.UnitOfWork.Common;
using Devon4Net.Domain.UnitOfWork.Repository;
using Devon4Net.WebAPI.Implementation.Business.SearchCriteria;
using Devon4Net.WebAPI.Implementation.Domain.Database;
using Devon4Net.WebAPI.Implementation.Domain.Entities;
using Devon4Net.WebAPI.Implementation.Domain.RepositoryInterfaces;
using System;
using System.Collections.Generic;
using System.Text;
using System.Threading.Tasks;

namespace Devon4Net.WebAPI.Implementation.Data.Repositories
{
    public class QueueRepository : Repository<Dailyqueue>, IQueueRepository
    {
        public QueueRepository(jtqdbContext context) : base(context) { }

        public async Task<Dailyqueue> Create(Business.QueueManagement.ETO.DailyqueueETO queue)
        {
            return await Create(new Dailyqueue
            {
                Modificationcounter = queue.modificationCounter,
                Name = queue.name,
                Logo = queue.logo,
                Currentnumber = queue.currentNumber,
                Minattentiontime = queue.minAttentionTime,
                Attentiontime = queue.attentionTime,
                Active = queue.active
            }).ConfigureAwait(false);
        }

        public async Task<bool> Delete(long id)
        {
            if(await GetFirstOrDefault(x => x.Id == id).ConfigureAwait(false) == null) { return false; }
            return await Delete(x => x.Id == id).ConfigureAwait(false);
        }

        public async Task<Dailyqueue> GetQueueById(long id)
        {
            return await GetFirstOrDefault(x => x.Id == id).ConfigureAwait(false);
        }

        public async Task<IList<Dailyqueue>> SearchByCriteria(QueueSearchCriteriaTo criteria)
        {
            var pre = PredicateBuilder.True<Dailyqueue>();

            if (!string.IsNullOrEmpty(criteria.Name))
            {
                pre = pre.And(x => x.Name == criteria.Name);
            }
            if (!string.IsNullOrEmpty(criteria.logo))
            {
                pre = pre.And(x => x.Logo == criteria.logo);
            }
            if (criteria.Modificationcounter != null)
            {
                pre = pre.And(x => x.Modificationcounter == criteria.Modificationcounter);
            }
            if (criteria.Minattentiontime != null)
            {
                pre = pre.And(x => x.Minattentiontime == criteria.Minattentiontime);
            }
            if (criteria.Attentiontime != null)
            {
                pre = pre.And(x => x.Attentiontime == criteria.Attentiontime);
            }
            if (criteria.Active != null)
            {
                pre = pre.And(x => x.Active == criteria.Active);
            }

            return await Get(pre).ConfigureAwait(false);
        }
    }
}
