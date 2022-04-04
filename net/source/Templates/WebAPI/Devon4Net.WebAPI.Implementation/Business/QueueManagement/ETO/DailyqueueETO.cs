using Devon4Net.WebAPI.Implementation.Business.AccessCodeManagement.ETO;
using System;
using System.Collections.Generic;

namespace Devon4Net.WebAPI.Implementation.Business.QueueManagement.ETO
{
    public partial class DailyqueueETO
    {

        public long? id { get; set; }
        public int modificationCounter { get; set; }
        public string name { get; set; }
        public string logo { get; set; }
        public string currentNumber { get; set; }
        public DateTime? attentionTime { get; set; }
        public DateTime minAttentionTime { get; set; }
        public bool? active { get; set; }
    }
}
