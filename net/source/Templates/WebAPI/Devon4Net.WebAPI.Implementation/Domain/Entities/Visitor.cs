using System;
using System.Collections.Generic;

namespace Devon4Net.WebAPI.Implementation.Domain.Entities
{
    public partial class Visitor
    {
        public Visitor()
        {
            Accesscode = new HashSet<Accesscode>();
        }

        public long Id { get; set; }
        public int Modificationcounter { get; set; }
        public string Username { get; set; }
        public string Name { get; set; }
        public string Password { get; set; }
        public string Phonenumber { get; set; }
        public bool? Acceptedcommercial { get; set; }
        public bool Acceptedterms { get; set; }
        public bool? Usertype { get; set; }

        public virtual ICollection<Accesscode> Accesscode { get; set; }
    }
}
