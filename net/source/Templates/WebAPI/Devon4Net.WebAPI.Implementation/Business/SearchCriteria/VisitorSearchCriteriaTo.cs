using System;
using System.Collections.Generic;
using System.Text;

namespace Devon4Net.WebAPI.Implementation.Business.SearchCriteria
{
    public class VisitorSearchCriteriaTo : AbstractSearchCriteriaTo
    {
        public static long serialVersionUID = 1L;
        public string username { get; set; }
        public string name { get; set; }
        public string phoneNumber { get; set; }
        public string password { get; set; }
        public bool? acceptedCommercial { get; set; }
        public bool? acceptedTerms { get; set; }
        public bool? userType { get; set; }
    }
}
