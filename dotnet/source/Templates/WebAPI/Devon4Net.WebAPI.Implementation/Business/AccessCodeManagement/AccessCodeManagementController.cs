using Devon4Net.WebAPI.Implementation.Business.AccessCodeManagement.Services;
using Devon4Net.WebAPI.Implementation.Business.AccessCodeManagement.ETO;
using Microsoft.AspNetCore.Cors;
using Microsoft.AspNetCore.Mvc;
using Newtonsoft.Json;
using System.Threading.Tasks;
using System;
using Microsoft.AspNetCore.Http;

namespace Devon4Net.WebAPI.Implementation.Business.AccessCodeManagement
{
    //[EnableCors("CorsPolicy")]
    [ApiController]
    [Route("jumpthequeue/services/rest/accesscodemanagement/v1/")]
    public class AccessCodeManagementController : ControllerBase
    {
        private readonly IAccessCodeManagementService _accessCodeManagementService;

        public AccessCodeManagementController(IAccessCodeManagementService accessCodeManagementService)
        {
            _accessCodeManagementService = accessCodeManagementService;
        }

        [HttpPost]
        [Route("accesscode/")]
        [ProducesResponseType(typeof(AccesscodeETO), StatusCodes.Status200OK)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status400BadRequest)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status401Unauthorized)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status404NotFound)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status500InternalServerError)]
        public async Task<ActionResult> JoinQueue([FromBody] AccessCodePostData postData)
        {
            var result = await _accessCodeManagementService.JoinQueueLogic(postData.visitorId, postData.queueId).ConfigureAwait(false);

            return Ok(result);
        }

        [HttpDelete]
        [Route("accesscode/{id}/")]
        [ProducesResponseType(typeof(bool), StatusCodes.Status200OK)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status400BadRequest)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status401Unauthorized)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status404NotFound)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status500InternalServerError)]
        public async Task<ActionResult> LeaveQueue(long id)
        {
            var result = await _accessCodeManagementService.LeaveQueueLogic(id).ConfigureAwait(false);
            if (!result)
            {
                return NotFound(null);
            }
            return Ok(id);
        }

        [HttpGet]
        [Route("accesscode/{id}/")]
        [ProducesResponseType(typeof(AccesscodeETO), StatusCodes.Status200OK)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status400BadRequest)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status401Unauthorized)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status404NotFound)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status500InternalServerError)]
        public async Task<ActionResult> GetById(long id)
        {
            var result = await _accessCodeManagementService.GetById(id).ConfigureAwait(false);
            if (result == null)
            {
                return NotFound(null);
            }
            return Ok(result);
        }
        
        [HttpPost]
        [Route("accesscode/search")]
        [ProducesResponseType(typeof(SearchCriteria.SearchResult), StatusCodes.Status200OK)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status400BadRequest)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status401Unauthorized)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status404NotFound)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status500InternalServerError)]
        public async Task<ActionResult> SearchAccessCodes([FromBody] SearchCriteria.AccessCodeSearchCriteriaTo criteria)
        {
            var searchResult = await _accessCodeManagementService.GetByCriteria(criteria).ConfigureAwait(false);
            return Ok(searchResult);
        }

        [HttpPost]
        [Route("accesscode/cto/search")]
        [ProducesResponseType(typeof(SearchCriteria.SearchResult), StatusCodes.Status200OK)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status400BadRequest)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status401Unauthorized)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status404NotFound)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status500InternalServerError)]
        public async Task<ActionResult> SearchAccessCodesCTO([FromBody] SearchCriteria.AccessCodeSearchCriteriaTo criteria)
        {
            var searchResult = await _accessCodeManagementService.GetByCriteriaCTO(criteria).ConfigureAwait(false);

            return Ok(searchResult);
        }
    }
}
