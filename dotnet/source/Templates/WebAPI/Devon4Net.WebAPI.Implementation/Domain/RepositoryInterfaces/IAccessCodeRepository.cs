using Devon4Net.Domain.UnitOfWork.Repository;
using Devon4Net.WebAPI.Implementation.Business.SearchCriteria;
using Devon4Net.WebAPI.Implementation.Domain.Entities;
using System.Collections.Generic;
using System.Threading.Tasks;

namespace Devon4Net.WebAPI.Implementation.Domain.RepositoryInterfaces
{
    public interface IAccessCodeRepository : IRepository<Accesscode>
    {
        Task<Accesscode> JoinQueue(string Ticketnumber, long Idvisitor, long Idqueue);
        Task<bool> LeaveQueue(long id);
        Task<Accesscode> GetById(long id);
        Task<IList<Accesscode>> GetAccessCodesBySearchCriteria(AccessCodeSearchCriteriaTo criteria);
        Task<IList<Business.CTO>> GetAccessCodesBySearchCriteriaCTO(AccessCodeSearchCriteriaTo criteria);
    }
}
