using Devon4Net.WebAPI.Implementation.Domain.Entities;
using System;
using System.Collections.Generic;
using System.Text;

namespace Devon4Net.WebAPI.Implementation.Business.VisitorManagement.Converters
{
    public class VisitorManagementConverter
    {
        public static ETO.VisitorETO ModelToETO(Visitor visitor)
        {
            if (visitor == null) return new ETO.VisitorETO();

            return new ETO.VisitorETO
            {
                id = visitor.Id,
                modificationCounter = visitor.Modificationcounter,
                name = visitor.Name,
                phoneNumber = visitor.Phonenumber,
                username = visitor.Username,
                password = visitor.Password,
                acceptedCommercial = visitor.Acceptedcommercial,
                acceptedTerms = visitor.Acceptedterms,
                userType = visitor.Usertype
            };
        }
    }
}
