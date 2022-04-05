using Devon4Net.WebAPI.Implementation.Business.QueueManagement.ETO;
using System;
using System.Collections.Generic;
using System.Text;
using System.Threading.Tasks;

namespace Devon4Net.WebAPI.Implementation.Business.QueueManagement.Services
{
    public interface IQueueManagementService
    {
        Task<DailyqueueETO> CreateQueue(DailyqueueETO queue);
        Task<bool> DeleteQueue(long id);
        Task<DailyqueueETO> GetById(long id);
        Task<SearchCriteria.SearchResult> SearchByCriteria(SearchCriteria.QueueSearchCriteriaTo criteria);
    }
}
