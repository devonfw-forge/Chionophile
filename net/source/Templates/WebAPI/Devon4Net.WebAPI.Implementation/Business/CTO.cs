using Devon4Net.WebAPI.Implementation.Domain.Entities;
using System;
using System.Collections.Generic;
using System.Text;

namespace Devon4Net.WebAPI.Implementation.Business
{
    public class CTO
    {
        public Accesscode accessCode { get; set; }
        public Dailyqueue queue { get; set; }
        public Visitor visitor{ get; set; }
    }
}
