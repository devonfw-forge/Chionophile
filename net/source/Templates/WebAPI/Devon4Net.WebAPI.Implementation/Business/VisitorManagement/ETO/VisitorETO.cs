using Devon4Net.WebAPI.Implementation.Business.AccessCodeManagement.ETO;
using System;
using System.Collections.Generic;
using Newtonsoft.Json;

namespace Devon4Net.WebAPI.Implementation.Business.VisitorManagement.ETO
{
    public partial class VisitorETO
    {
        public long id { get; set; }
        public int modificationCounter { get; set; }
        public string username { get; set; }
        public string name { get; set; }
        public string password { get; set; }
        public string phoneNumber { get; set; }
        public bool? acceptedCommercial { get; set; }
        public bool acceptedTerms { get; set; }
        public bool? userType { get; set; }
    }
}
