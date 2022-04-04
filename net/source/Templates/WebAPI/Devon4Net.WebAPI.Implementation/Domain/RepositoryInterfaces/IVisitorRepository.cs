using Devon4Net.Domain.UnitOfWork.Repository;
using Devon4Net.WebAPI.Implementation.Business.SearchCriteria;
using Devon4Net.WebAPI.Implementation.Business.VisitorManagement.ETO;
using Devon4Net.WebAPI.Implementation.Domain.Entities;
using System;
using System.Collections.Generic;
using System.Text;
using System.Threading.Tasks;

namespace Devon4Net.WebAPI.Implementation.Domain.RepositoryInterfaces
{
    public interface IVisitorRepository : IRepository<Visitor>
    {
        Task<Visitor> Create(VisitorETO visitor);
        Task<bool> Delete(long id);
        Task<Visitor> GetVisitorById(long id);
        Task<IList<Visitor>> GetVisitorBySearchCriteria(VisitorSearchCriteriaTo criteria);
    }
}
