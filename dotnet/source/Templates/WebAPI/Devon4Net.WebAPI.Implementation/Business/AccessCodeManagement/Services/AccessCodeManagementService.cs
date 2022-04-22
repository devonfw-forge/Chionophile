using Devon4Net.Domain.UnitOfWork.Service;
using Devon4Net.Domain.UnitOfWork.UnitOfWork;
using Devon4Net.WebAPI.Implementation.Business.AccessCodeManagement.Converters;
using Devon4Net.WebAPI.Implementation.Business.SearchCriteria;
using Devon4Net.WebAPI.Implementation.Domain.Database;
using Devon4Net.WebAPI.Implementation.Domain.RepositoryInterfaces;
using System.Threading.Tasks;
using Devon4Net.WebAPI.Implementation.Business.AccessCodeManagement.ETO;
using System.Collections.Generic;
using System;
using System.Threading;

namespace Devon4Net.WebAPI.Implementation.Business.AccessCodeManagement.Services
{
    public class AccessCodeManagementService : Service<jtqdbContext>, IAccessCodeManagementService
    {
        private readonly IAccessCodeRepository _accessCodeRepository;
        //private static readonly Mutex _lock = new Mutex();

        public AccessCodeManagementService(IUnitOfWork<jtqdbContext> uoW) : base(uoW)
        {
            _accessCodeRepository = uoW.Repository<IAccessCodeRepository>();
        }

        public async Task<SearchResult> GetByCriteria(AccessCodeSearchCriteriaTo criteria)
        {
            var accessCodes = await _accessCodeRepository.GetAccessCodesBySearchCriteria(criteria).ConfigureAwait(false);
            var accessCodesETO = new List<AccesscodeETO>();

            for (int i = 0; i < accessCodes.Count; i++)
            {
                accessCodesETO.Add(AccessCodeManagementConverter.ModelToETO(accessCodes[i]));
            }

            var pageStart = criteria.pageable.pageNumber * criteria.pageable.pageSize;
            var pageIncrement = criteria.pageable.pageSize;

            if(pageIncrement > accessCodesETO.Count)
            {
                pageIncrement = accessCodesETO.Count;
            }

            var result = new SearchResult
            {
                content = accessCodesETO.GetRange(pageStart, pageIncrement),
                pageable = criteria.pageable,
                count = accessCodesETO.Count
            };

            return result;
        }

        public async Task<SearchResult> GetByCriteriaCTO(AccessCodeSearchCriteriaTo criteria)
        {
            var accessCodes = await _accessCodeRepository.GetAccessCodesBySearchCriteriaCTO(criteria).ConfigureAwait(false);


            var response = new List<EntityCTO>(accessCodes);
            response.Reverse();

            var pageStart = criteria.pageable.pageNumber * criteria.pageable.pageSize;
            var pageIncrement = criteria.pageable.pageSize;

            if (pageIncrement > response.Count)
            {
                pageIncrement = response.Count;
            }
            var pagedResult = response.GetRange(pageStart, pageIncrement)
                .ConvertAll(AccessCodeManagementConverter.EntityToResponseCTO);

            var result = new SearchResult
            {
                content = pagedResult,
                pageable = criteria.pageable,
                count = response.Count
            };

            return result;
        }

        public async Task<AccesscodeETO> GetById(long id)
        {
            var result = await _accessCodeRepository.GetById(id).ConfigureAwait(false);
            if(result == null) { return null; }
            return AccessCodeManagementConverter.ModelToETO(result);
        }

        public async Task<AccesscodeETO> JoinQueueLogic(long visitorID, long queueID) 
        {
            var result = await _accessCodeRepository.JoinQueue(visitorID, queueID).ConfigureAwait(false);
            return AccessCodeManagementConverter.ModelToETO(result);
        }

        public async Task<bool> LeaveQueueLogic(long accessCodeID)
        {
            return await _accessCodeRepository.LeaveQueue(accessCodeID).ConfigureAwait(false);
        }


    }
}
