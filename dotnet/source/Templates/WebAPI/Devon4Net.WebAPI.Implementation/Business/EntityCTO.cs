using Devon4Net.WebAPI.Implementation.Domain.Entities;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Devon4Net.WebAPI.Implementation.Business
{
    public class EntityCTO
    {
        public Accesscode accescode { get; set; }
        public Dailyqueue queue { get; set; }
        public Visitor visitor { get; set; }
    }
}
