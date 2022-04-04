using System;
using System.Collections.Generic;

namespace Devon4Net.WebAPI.Implementation.Domain.Entities
{
    public partial class Dailyqueue
    {
        public Dailyqueue()
        {
            Accesscode = new HashSet<Accesscode>();
        }

        public long Id { get; set; }
        public int Modificationcounter { get; set; }
        public string Name { get; set; }
        public string Logo { get; set; }
        public string Currentnumber { get; set; }
        public DateTime? Attentiontime { get; set; }
        public DateTime Minattentiontime { get; set; }
        public bool? Active { get; set; }
        
        [Newtonsoft.Json.JsonIgnore]
        public virtual ICollection<Accesscode> Accesscode { get; set; }
    }
}
