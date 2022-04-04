using System;
using System.Collections.Generic;
using System.Text;

namespace Devon4Net.WebAPI.Implementation.Business.SearchCriteria
{
    public class QueueSearchCriteriaTo : AbstractSearchCriteriaTo
    {
        public static long serialVersionUID = 1L;
        public int? Modificationcounter { get; set; }
        public string Name { get; set; }
        public string logo { get; set; }
        public DateTime? Attentiontime { get; set; }
        public DateTime? Minattentiontime { get; set; }
        public bool? Active { get; set; }
    }
}
