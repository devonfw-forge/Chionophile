using Devon4Net.Domain.UnitOfWork.Repository;
using Devon4Net.WebAPI.Implementation.Domain.Entities;
using System.Collections.Generic;
using System.Threading.Tasks;

namespace Devon4Net.WebAPI.Implementation.Domain.RepositoryInterfaces
{
    public interface IQueueRepository : IRepository<Dailyqueue>
    {
        Task<Dailyqueue> Create(Business.QueueManagement.ETO.DailyqueueETO queue);
        Task<bool> Delete(long id);
        Task<Dailyqueue> GetQueueById(long id);
        Task<IList<Dailyqueue>> SearchByCriteria(Business.SearchCriteria.QueueSearchCriteriaTo criteria);
    }
}
