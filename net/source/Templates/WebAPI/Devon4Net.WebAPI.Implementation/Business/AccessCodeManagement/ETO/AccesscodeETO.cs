using Devon4Net.WebAPI.Implementation.Business.QueueManagement.ETO;
using Devon4Net.WebAPI.Implementation.Business.VisitorManagement.ETO;
using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Devon4Net.WebAPI.Implementation.Business.AccessCodeManagement.ETO
{
    public partial class AccesscodeETO
    {
        
        //public int modificationCounter { get; set; }
        public long id { get; set; }
        public string ticketNumber { get; set; }
        public DateTime creationTime { get; set; }
        public DateTime? startTime { get; set; }
        public DateTime? endTime { get; set; }
        public long queueId { get; set; }
        public long visitorId { get; set; }
        
        
        
    }
}
