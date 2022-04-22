using Devon4Net.Domain.UnitOfWork.Common;
using Devon4Net.Domain.UnitOfWork.Repository;
using Devon4Net.WebAPI.Implementation.Business.SearchCriteria;
using Devon4Net.WebAPI.Implementation.Domain.Database;
using Devon4Net.WebAPI.Implementation.Domain.Entities;
using Devon4Net.WebAPI.Implementation.Domain.RepositoryInterfaces;
using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using System.Linq;

namespace Devon4Net.WebAPI.Implementation.Data.Repositories
{
    public class AccessCodeRepository : Repository<Accesscode>, IAccessCodeRepository
    {
        public AccessCodeRepository(jtqdbContext context) : base(context) { }

        public async Task<Accesscode> JoinQueue(long Idvisitor, long Idqueue)
        {
            return await Create(new Accesscode 
            {
                Modificationcounter = 1, 
                Creationtime = DateTime.Now,
                Starttime = null,
                Endtime = null,
                Idvisitor = Idvisitor,
                Idqueue = Idqueue
            }).ConfigureAwait(false);
        }

        public async Task<bool> LeaveQueue(long id)
        {
            if(await GetFirstOrDefault(t => t.Id == id).ConfigureAwait(false) == null) { return false; }
            return await Delete(t => t.Id == id).ConfigureAwait(false);
        }

        public async Task<IList<Accesscode>> GetAccessCodesBySearchCriteria(AccessCodeSearchCriteriaTo criteria)
        {
            var pre = PredicateBuilder.True<Accesscode>();

            if (criteria.Modificationcounter != null)
            {
                pre = pre.And(x => x.Modificationcounter == criteria.Modificationcounter);
            }
            if (criteria.Creationtime != null)
            {
                pre = pre.And(x => x.Creationtime == criteria.Creationtime);
            }
            if (criteria.Starttime != null)
            {
                pre = pre.And(x => x.Starttime == criteria.Starttime);
            }
            if (criteria.Endtime != null)
            {
                pre = pre.And(x => x.Endtime == criteria.Endtime);
            }
            if (criteria.Idvisitor != null)
            {
                pre = pre.And(x => x.Idvisitor == criteria.Idvisitor);
            }
            if (criteria.Idqueue != null)
            {
                pre = pre.And(x => x.Idqueue == criteria.Idqueue);
            }

            return await Get(pre).ConfigureAwait(false);
        }

        public async Task<Accesscode> GetById(long id)
        {
            return await GetFirstOrDefault(x => x.Id == id).ConfigureAwait(false);
        }

        public async Task<IList<Business.EntityCTO>> GetAccessCodesBySearchCriteriaCTO(AccessCodeSearchCriteriaTo criteria)
        {
            var db = new jtqdbContext();

            var pre = PredicateBuilder.True<Accesscode>();

            if (criteria.Modificationcounter != null)
            {
                pre = pre.And(x => x.Modificationcounter == criteria.Modificationcounter);
            }
            if (criteria.Creationtime != null)
            {
                pre = pre.And(x => x.Creationtime == criteria.Creationtime);
            }
            if (criteria.Starttime != null)
            {
                pre = pre.And(x => x.Starttime == criteria.Starttime);
            }
            if (criteria.Endtime != null)
            {
                pre = pre.And(x => x.Endtime == criteria.Endtime);
            }
            if (criteria.Idvisitor != null)
            {
                pre = pre.And(x => x.Idvisitor == criteria.Idvisitor);
            }
            if (criteria.Idqueue != null)
            {
                pre = pre.And(x => x.Idqueue == criteria.Idqueue);
            }

            var search = await Get(pre).ConfigureAwait(false);

            var query = from ac in db.Accesscode
                        join v in db.Visitor on ac.Idvisitor equals v.Id
                        join q in db.Dailyqueue on ac.Idqueue equals q.Id
                        select new Business.EntityCTO
                        {
                            accescode = ac,
                            visitor = v,
                            queue = q
                        };
            //var a = query.Where();

            var result = new List<Business.EntityCTO>(query);

            return result;
        }
    }
}
