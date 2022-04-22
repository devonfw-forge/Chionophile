using System;
using System.Collections;
using System.Collections.Generic;
using System.Text;

namespace Devon4Net.WebAPI.Implementation.Business.SearchCriteria
{
    public class SearchResult
    {
        public IList content { get; set; }
        public IPageable pageable { get; set; }
        public int count { get; set; }
    }
}
