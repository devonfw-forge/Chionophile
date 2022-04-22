using Devon4Net.Domain.UnitOfWork.Common;
using Devon4Net.Domain.UnitOfWork.Repository;
using Devon4Net.WebAPI.Implementation.Business.SearchCriteria;
using Devon4Net.WebAPI.Implementation.Business.VisitorManagement.ETO;
using Devon4Net.WebAPI.Implementation.Domain.Database;
using Devon4Net.WebAPI.Implementation.Domain.Entities;
using Devon4Net.WebAPI.Implementation.Domain.RepositoryInterfaces;
using System;
using System.Collections.Generic;
using System.Threading.Tasks;

namespace Devon4Net.WebAPI.Implementation.Data.Repositories
{
    public class VisitorRepository : Repository<Visitor>, IVisitorRepository
    {
        public VisitorRepository(jtqdbContext context) : base(context) { }

        public async Task<Visitor> Create(VisitorETO visitor)
        {
            return await Create(new Visitor {
                Modificationcounter = visitor.modificationCounter,
                Name = visitor.name,
                Phonenumber = visitor.phoneNumber,
                Username = visitor.username,
                Password = visitor.password,
                Acceptedcommercial = visitor.acceptedCommercial,
                Acceptedterms = visitor.acceptedTerms,
                Usertype = visitor.userType
            }).ConfigureAwait(false);
        }

        public async Task<bool> Delete(long id)
        {
            if (await GetFirstOrDefault(t => t.Id == id) == null) { return false; }
            return await Delete(t => t.Id == id).ConfigureAwait(false);
        }

        public async Task<Visitor> GetVisitorById(long id)
        {
            return await GetFirstOrDefault(t => t.Id == id).ConfigureAwait(false);
        }

        public async Task<IList<Visitor>> GetVisitorBySearchCriteria(VisitorSearchCriteriaTo criteria)
        {
            var pre = PredicateBuilder.True<Visitor>();

            if (!string.IsNullOrEmpty(criteria.name))
            {
                pre = pre.And(x => x.Name == criteria.name);
            }
            if (!string.IsNullOrEmpty(criteria.phoneNumber))
            {
                pre = pre.And(x => x.Phonenumber == criteria.phoneNumber);
            }
            if (!string.IsNullOrEmpty(criteria.username))
            {
                pre = pre.And(x => x.Username == criteria.username);
            }
            if (!string.IsNullOrEmpty(criteria.password))
            {
                pre = pre.And(x => x.Password == criteria.password);
            }
            if (criteria.acceptedCommercial != null)
            {
                pre = pre.And(x => x.Acceptedcommercial == criteria.acceptedCommercial);
            }
            if (criteria.acceptedTerms != null)
            {
                pre = pre.And(x => x.Acceptedterms == criteria.acceptedTerms);
            }
            if (criteria.userType != null)
            {
                pre = pre.And(x => x.Usertype == criteria.userType);
            }

            return await Get(pre).ConfigureAwait(false);
        }
    }
}
