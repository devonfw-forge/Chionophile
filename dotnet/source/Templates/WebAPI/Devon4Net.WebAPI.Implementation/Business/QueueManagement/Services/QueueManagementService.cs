using Devon4Net.Domain.UnitOfWork.UnitOfWork;
using Devon4Net.Domain.UnitOfWork.Service;
using Devon4Net.WebAPI.Implementation.Business.QueueManagement.ETO;
using Devon4Net.WebAPI.Implementation.Business.SearchCriteria;
using Devon4Net.WebAPI.Implementation.Domain.Database;
using Devon4Net.WebAPI.Implementation.Domain.RepositoryInterfaces;
using System;
using System.Collections.Generic;
using System.Text;
using System.Threading.Tasks;
using Devon4Net.WebAPI.Implementation.Business.QueueManagement.Converters;

namespace Devon4Net.WebAPI.Implementation.Business.QueueManagement.Services
{
    public class QueueManagementService : Service<jtqdbContext>, IQueueManagementService
    {
        private readonly IQueueRepository _queueRepository;
        public QueueManagementService(IUnitOfWork<jtqdbContext> uoW) : base(uoW)
        {
            _queueRepository = uoW.Repository<IQueueRepository>();
        }

        public async Task<DailyqueueETO> CreateQueue(DailyqueueETO queue)
        {
            var result = await _queueRepository.Create(queue).ConfigureAwait(false);
            return QueueManagementConverter.ModelToETO(result);
        }

        public async Task<bool> DeleteQueue(long id)
        {
            return await _queueRepository.Delete(id).ConfigureAwait(false);
        }

        public async Task<DailyqueueETO> GetById(long id)
        {
            var result = await _queueRepository.GetQueueById(id).ConfigureAwait(false);
            if(result == null) { return null; }
            return QueueManagementConverter.ModelToETO(result);
        }

        public async Task<SearchResult> SearchByCriteria(QueueSearchCriteriaTo criteria)
        {
            var queues = await _queueRepository.SearchByCriteria(criteria).ConfigureAwait(false);
            var queuesETO = new List<DailyqueueETO>();

            foreach (var queue in queues)
            {
                queuesETO.Add(QueueManagementConverter.ModelToETO(queue));
            }

            var pageStart = criteria.pageable.pageNumber * criteria.pageable.pageSize;
            var pageIncrement = criteria.pageable.pageSize;

            if (pageIncrement > queuesETO.Count)
            {
                pageIncrement = queuesETO.Count;
            }

            var result = new SearchResult
            {
                content = queuesETO.GetRange(pageStart, pageIncrement),
                pageable = criteria.pageable,
                count = queuesETO.Count
            };

            return result;
        }
    }
}
