using Devon4Net.WebAPI.Implementation.Domain.Entities;
using Newtonsoft.Json.Converters;
using System;
using System.Collections.Generic;
using System.Text;

namespace Devon4Net.WebAPI.Implementation.Business.AccessCodeManagement.Converters
{
    public class AccessCodeManagementConverter
    {
        public static ETO.AccesscodeETO ModelToETO(Accesscode accesscode) 
        {
            if (accesscode == null) return new ETO.AccesscodeETO();

            if (accesscode.Endtime != null) { accesscode.Endtime = accesscode.Endtime.Value.ToUniversalTime(); }
            if (accesscode.Starttime != null) { accesscode.Starttime = accesscode.Starttime.Value.ToUniversalTime(); }
            
            return new ETO.AccesscodeETO
            {
                //modificationCounter = accesscode.Modificationcounter,
                id = accesscode.Id,
                ticketNumber = accesscode.Ticketnumber,
                creationTime = accesscode.Creationtime.ToUniversalTime(),
                startTime = accesscode.Starttime,
                endTime = accesscode.Endtime,
                visitorId = accesscode.Idvisitor,
                queueId = accesscode.Idqueue
            };
        }
    }
}
