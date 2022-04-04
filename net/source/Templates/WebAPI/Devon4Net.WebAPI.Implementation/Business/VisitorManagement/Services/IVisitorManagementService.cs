using Devon4Net.WebAPI.Implementation.Business.SearchCriteria;
using Devon4Net.WebAPI.Implementation.Business.VisitorManagement.ETO;
using System;
using System.Collections.Generic;
using System.Text;
using System.Threading.Tasks;

namespace Devon4Net.WebAPI.Implementation.Business.VisitorManagement.Services
{
    public interface IVisitorManagementService
    {
        Task<VisitorETO> Save(VisitorETO visitor);
        Task<bool> Delete(long id);
        Task<VisitorETO> FindById(long id);
        Task<SearchResult> FindByCriteria(VisitorSearchCriteriaTo criteria);
        bool EmailVerification(string email);
        bool PhoneVerification(string phone);
    }
}
