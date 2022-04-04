using System.Collections.Generic;
using System.Threading.Tasks;
using Devon4Net.WebAPI.Implementation.Business.AccessCodeManagement.ETO;

namespace Devon4Net.WebAPI.Implementation.Business.AccessCodeManagement.Services
{
    public interface IAccessCodeManagementService
    {
        Task<AccesscodeETO> JoinQueueLogic(long visitorID, long queueID);
        Task<bool> LeaveQueueLogic(long accessCodeID);
        Task<AccesscodeETO> GetById(long id);
        Task<SearchCriteria.SearchResult> GetByCriteria(SearchCriteria.AccessCodeSearchCriteriaTo criteria);
        Task<SearchCriteria.SearchResult> GetByCriteriaCTO(SearchCriteria.AccessCodeSearchCriteriaTo criteria);
    }
}
