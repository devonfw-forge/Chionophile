using System;
using System.Collections.Generic;
using System.Text;

namespace Devon4Net.WebAPI.Implementation.Business.SearchCriteria
{
    public abstract class AbstractSearchCriteriaTo
    {
        public IPageable pageable { get; set; }
    }
}
