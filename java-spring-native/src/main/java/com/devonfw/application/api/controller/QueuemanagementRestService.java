package com.devonfw.application.api.controller;

import javax.ws.rs.Consumes;
import javax.ws.rs.DELETE;
import javax.ws.rs.GET;
import javax.ws.rs.POST;
import javax.ws.rs.Path;
import javax.ws.rs.PathParam;
import javax.ws.rs.Produces;
import javax.ws.rs.core.MediaType;

import com.devonfw.application.domain.tos.QueueSearchCriteriaTo;
import org.springframework.data.domain.Page;

import com.devonfw.application.api.model.QueueEto;

/**
 * The service interface for REST calls in order to execute the logic of
 * component {@link Queuemanagement}.
 */
@Path("/queuemanagement/v1")
@Consumes(MediaType.APPLICATION_JSON)
@Produces(MediaType.APPLICATION_JSON)
public interface QueuemanagementRestService {

  /**
   * Delegates to {@link Queuemanagement#findQueue}.
   *
   * @param id the ID of the {@link QueueEto}
   * @return the {@link QueueEto}
   */
  @GET
  @Path("/queue/{id}/")
  public QueueEto getQueue(@PathParam("id") long id);

  /**
   * Delegates to {@link Queuemanagement#saveQueue}.
   *
   * @param queue the {@link QueueEto} to be saved
   * @return the recently created {@link QueueEto}
   */
  @POST
  @Path("/queue/")
  public QueueEto saveQueue(QueueEto queue);

  /**
   * Delegates to {@link Queuemanagement#deleteQueue}.
   *
   * @param id ID of the {@link QueueEto} to be deleted
   * @return id
   */
  @DELETE
  @Path("/queue/{id}/")
  public long deleteQueue(@PathParam("id") long id);

  /**
   * Delegates to {@link Queuemanagement#findQueueEtos}.
   *
   * @param searchCriteriaTo the pagination and search criteria to be used for
   *                         finding queues.
   * @return the {@link Page list} of matching {@link QueueEto}s.
   */
  @Path("/queue/search")
  @POST
  public Page<QueueEto> findQueues(QueueSearchCriteriaTo searchCriteriaTo);

}