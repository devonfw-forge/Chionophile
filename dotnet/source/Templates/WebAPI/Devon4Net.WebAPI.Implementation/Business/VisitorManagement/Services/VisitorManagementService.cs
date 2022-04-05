using Devon4Net.Domain.UnitOfWork.Service;
using Devon4Net.Domain.UnitOfWork.UnitOfWork;
using Devon4Net.WebAPI.Implementation.Business.SearchCriteria;
using Devon4Net.WebAPI.Implementation.Business.VisitorManagement.Converters;
using Devon4Net.WebAPI.Implementation.Business.VisitorManagement.ETO;
using Devon4Net.WebAPI.Implementation.Domain.Database;
using Devon4Net.WebAPI.Implementation.Domain.RepositoryInterfaces;
using System;
using System.Collections.Generic;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace Devon4Net.WebAPI.Implementation.Business.VisitorManagement.Services
{
    public class VisitorManagementService : Service<jtqdbContext>, IVisitorManagementService
    {
        private readonly IVisitorRepository _visitorRepository;

        public VisitorManagementService(IUnitOfWork<jtqdbContext> uoW) : base(uoW)
        {
            _visitorRepository = uoW.Repository<IVisitorRepository>();
        }

        public async Task<VisitorETO> Save(VisitorETO visitor)
        {
            var result = await _visitorRepository.Create(visitor).ConfigureAwait(false);

            return VisitorManagementConverter.ModelToETO(result);
        }

        public async Task<bool> Delete(long id)
        {
            return await _visitorRepository.Delete(id).ConfigureAwait(false);
        }

        public async Task<SearchResult> FindByCriteria(VisitorSearchCriteriaTo criteria)
        {
            var visitors = await _visitorRepository.GetVisitorBySearchCriteria(criteria).ConfigureAwait(false);
            var visitorsETO = new List<VisitorETO>();

            foreach (var visitor in visitors)
            {
                visitorsETO.Add(VisitorManagementConverter.ModelToETO(visitor));
            }

            var pageStart = criteria.pageable.pageNumber * criteria.pageable.pageSize;
            var pageIncrement = criteria.pageable.pageSize;

            if (pageIncrement > visitorsETO.Count)
            {
                pageIncrement = visitorsETO.Count;
            }

            var result = new SearchResult
            {
                result = visitorsETO.GetRange(pageStart, pageIncrement),
                pageable = criteria.pageable,
                count = visitorsETO.Count
            };

            return result;
        }

        public async Task<VisitorETO> FindById(long id)
        {
            var result = await _visitorRepository.GetVisitorById(id).ConfigureAwait(false);
            return VisitorManagementConverter.ModelToETO(result);
        }

        public bool EmailVerification(string email)
        {
            return Regex.IsMatch(email, @"\A(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?)\Z", RegexOptions.IgnoreCase); ;
        }
        public bool PhoneVerification(string phone)
        {
            if (!string.IsNullOrEmpty(phone))
            {
                if (phone.Length == 9 && int.TryParse(phone, out _))
                {
                    return true;
                }
            }

            return false;
        }

    }
}
