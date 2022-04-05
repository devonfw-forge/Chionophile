using System;
using System.Collections.Generic;
using System.Text;

namespace Devon4Net.WebAPI.Implementation.Business.SearchCriteria
{
    public class IPageable
    {
        public int pageNumber { get; set; }
        public int pageSize { get; set; }
        public string[] sort { get; set; }
    }
}
