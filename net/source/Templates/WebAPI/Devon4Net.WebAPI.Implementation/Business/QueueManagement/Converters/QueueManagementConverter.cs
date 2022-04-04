using Devon4Net.WebAPI.Implementation.Domain.Entities;
using System;
using System.Collections.Generic;
using System.Text;

namespace Devon4Net.WebAPI.Implementation.Business.QueueManagement.Converters
{
    public class QueueManagementConverter
    {
        public static ETO.DailyqueueETO ModelToETO(Dailyqueue queue)
        {
            if (queue == null) return new ETO.DailyqueueETO();

            return new ETO.DailyqueueETO
            {
                id = queue.Id,
                modificationCounter = queue.Modificationcounter,
                name = queue.Name,
                logo = queue.Logo,
                currentNumber = queue.Currentnumber,
                minAttentionTime = queue.Minattentiontime,
                attentionTime = queue.Attentiontime,
                active = queue.Active
            };
        }
    }
}
