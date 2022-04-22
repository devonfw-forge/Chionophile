using Devon4Net.WebAPI.Implementation.Business.AccessCodeManagement.ETO;
using Devon4Net.WebAPI.Implementation.Business.QueueManagement.ETO;
using Devon4Net.WebAPI.Implementation.Business.VisitorManagement.ETO;
using Devon4Net.WebAPI.Implementation.Domain.Entities;
using System;
using System.Collections.Generic;
using System.Text;

namespace Devon4Net.WebAPI.Implementation.Business
{
    public class CTOResponse
    {
        public AccesscodeETO accessCode { get; set; }
        public DailyqueueETO queue { get; set; }
        public VisitorETO visitor{ get; set; }
    }
}
