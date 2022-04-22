using System;
using System.Collections.Generic;
using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;

namespace Devon4Net.WebAPI.Implementation.Domain.Entities
{
    public partial class Accesscode
    {
        public long Id { get; set; }
        public int Modificationcounter { get; set; }
        public DateTime Creationtime { get; set; }
        public DateTime? Starttime { get; set; }
        public DateTime? Endtime { get; set; }
        public long Idvisitor { get; set; }
        public long Idqueue { get; set; }

        public virtual Dailyqueue IdqueueNavigation { get; set; }
        public virtual Visitor IdvisitorNavigation { get; set; }
    }
}
