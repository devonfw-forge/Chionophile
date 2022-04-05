using System;
using System.Collections.Generic;
using System.Runtime.Serialization;
using System.Text;

namespace Devon4Net.WebAPI.Implementation.Business
{
    [DataContract]
    public class GenericHttpResponseError
    {
        [DataMember(Name = "code")]
        public int Code { get; set; }

        [DataMember(Name = "message")]
        public string Message { get; set; }
    }
}
