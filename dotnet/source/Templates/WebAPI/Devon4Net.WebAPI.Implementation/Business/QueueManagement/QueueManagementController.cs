using Devon4Net.WebAPI.Implementation.Business.QueueManagement.ETO;
using Devon4Net.WebAPI.Implementation.Business.QueueManagement.Services;
using Microsoft.AspNetCore.Cors;
using Microsoft.AspNetCore.Http;
using Microsoft.AspNetCore.Mvc;
using Newtonsoft.Json;
using System;
using System.Collections.Generic;
using System.Text;
using System.Threading.Tasks;

namespace Devon4Net.WebAPI.Implementation.Business.QueueManagement
{
    //[EnableCors("CorsPolicy")]
    [ApiController]
    [Route("jumpthequeue/services/rest/queuemanagement/v1/")]
    public class QueueManagementController : ControllerBase
    {
        private readonly IQueueManagementService _queueManagementService;
        public QueueManagementController(IQueueManagementService queueManagementService)
        {
            _queueManagementService = queueManagementService;
        }

        [HttpPost]
        [Route("queue")]
        [ProducesResponseType(typeof(DailyqueueETO), StatusCodes.Status200OK)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status400BadRequest)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status401Unauthorized)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status404NotFound)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status500InternalServerError)]
        public async Task<ActionResult> CreateQueue([FromBody] DailyqueueETO queue)
        {
            var result = await _queueManagementService.CreateQueue(queue).ConfigureAwait(false);
            return Ok(result);
        }

        [HttpPost]
        [Route("queue/search")]
        [ProducesResponseType(typeof(SearchCriteria.SearchResult), StatusCodes.Status200OK)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status400BadRequest)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status401Unauthorized)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status404NotFound)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status500InternalServerError)]
        public async Task<ActionResult> SearchQueue([FromBody] SearchCriteria.QueueSearchCriteriaTo criteria)
        {
            var searchResult = await _queueManagementService.SearchByCriteria(criteria).ConfigureAwait(false);

            return Ok(searchResult);
        }

        [HttpGet]
        [Route("queue/{id}")]
        [ProducesResponseType(typeof(DailyqueueETO), StatusCodes.Status200OK)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status400BadRequest)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status401Unauthorized)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status404NotFound)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status500InternalServerError)]
        public async Task<ActionResult> GetQueue([FromRoute] long id)
        {
            var result = await _queueManagementService.GetById(id).ConfigureAwait(false);
            if (result == null)
            {
                return NotFound(null);
            }
            return Ok(result);
        }

        [HttpDelete]
        [Route("queue/{id}")]
        [ProducesResponseType(typeof(bool), StatusCodes.Status200OK)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status400BadRequest)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status401Unauthorized)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status404NotFound)]
        [ProducesResponseType(typeof(GenericHttpResponseError), StatusCodes.Status500InternalServerError)]
        public async Task<ActionResult> DeleteQueue([FromRoute] long id)
        {
            var result = await _queueManagementService.DeleteQueue(id).ConfigureAwait(false);
            if (!result)
            {
                return NotFound(null);
            }
            return Ok(id);
        }
    }
}
