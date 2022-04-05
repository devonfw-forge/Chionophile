using System;
using System.Collections.Generic;
using System.Text;

namespace Devon4Net.WebAPI.Implementation.Business.SearchCriteria
{
    public class AccessCodeSearchCriteriaTo : AbstractSearchCriteriaTo
    {
        public static long serialVersionUID = 1L;
        public int? Modificationcounter { get; set; }
        public string Ticketnumber { get; set; }
        public DateTime? Creationtime { get; set; }
        public DateTime? Starttime { get; set; }
        public DateTime? Endtime { get; set; }
        public long? Idvisitor { get; set; }
        public long? Idqueue { get; set; }
    }
}
