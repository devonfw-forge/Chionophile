using Devon4Net.Infrastructure.Log;
using Devon4Net.WebAPI.Implementation.Business.VisitorManagement.ETO;
using Devon4Net.WebAPI.Implementation.Business.VisitorManagement.Services;
using Microsoft.AspNetCore.Cors;
using Microsoft.AspNetCore.Http;
using Microsoft.AspNetCore.Mvc;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using Newtonsoft.Json.Serialization;
using System;
using System.Collections.Generic;
using System.Text;
using System.Threading.Tasks;

namespace Devon4Net.WebAPI.Implementation.Business.VisitorManagement
{
    [EnableCors("CorsPolicy")]
    [ApiController]
    [Route("jumpthequeue/services/rest/visitormanagement/v1")]
    public class VisitorManagementController : ControllerBase
    {
        private readonly IVisitorManagementService _visitorManagementService;

        public VisitorManagementController(IVisitorManagementService visitorManagementService)
        {
            _visitorManagementService = visitorManagementService;
        }

        [HttpPost]
        [Route("visitor")]
        [ProducesResponseType(typeof(VisitorETO), StatusCodes.Status200OK)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status400BadRequest)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status401Unauthorized)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status404NotFound)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status500InternalServerError)]
        public async Task<ActionResult> SaveVisitor([FromBody]VisitorETO visitor)
        {
            if (!_visitorManagementService.EmailVerification(visitor.username) || !_visitorManagementService.PhoneVerification(visitor.phoneNumber))
            {
                Response.StatusCode = 400;
                return Content("Invalid field");
            }

            var result = await _visitorManagementService.Save(visitor).ConfigureAwait(false);

            return Ok(result);
        }

        [HttpDelete]
        [Route("visitor/{id}")]
        [ProducesResponseType(typeof(long), StatusCodes.Status200OK)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status400BadRequest)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status401Unauthorized)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status404NotFound)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status500InternalServerError)]
        public async Task<ActionResult> DeleteVisitor(long id)
        {
            var result = await _visitorManagementService.Delete(id).ConfigureAwait(false);
            
            if (!result)
            {
                return NotFound(null);
            }

            return Ok(id);
        }

        [HttpGet]
        [Route("visitor/{id}")]
        [ProducesResponseType(typeof(VisitorETO), StatusCodes.Status200OK)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status400BadRequest)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status401Unauthorized)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status404NotFound)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status500InternalServerError)]
        public async Task<ActionResult> GetVisitorById(long id)
        {
            var result = await _visitorManagementService.FindById(id).ConfigureAwait(false);

            if (result == null)
            {
                return NotFound(null);
            }

            return Ok(result);
        }

        [HttpPost]
        [Route("visitor/search")]
        [ProducesResponseType(typeof(SearchCriteria.SearchResult), StatusCodes.Status200OK)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status400BadRequest)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status401Unauthorized)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status404NotFound)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status500InternalServerError)]
        public async Task<ActionResult> SearchVisitors([FromBody] SearchCriteria.VisitorSearchCriteriaTo criteria)
        {
            var searchResult = await _visitorManagementService.FindByCriteria(criteria).ConfigureAwait(false);
            return Ok(searchResult);
        }
    }
}
