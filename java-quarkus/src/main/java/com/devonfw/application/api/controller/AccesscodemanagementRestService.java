package com.devonfw.application.api.controller;

import javax.ws.rs.Consumes;
import javax.ws.rs.DELETE;
import javax.ws.rs.GET;
import javax.ws.rs.POST;
import javax.ws.rs.Path;
import javax.ws.rs.PathParam;
import javax.ws.rs.Produces;
import javax.ws.rs.core.MediaType;

import org.springframework.data.domain.Page;

import com.devonfw.application.api.model.AccessCodeCto;
import com.devonfw.application.api.model.AccessCodeEto;
import com.devonfw.application.domain.tos.AccessCodeSearchCriteriaTo;

/**
 * The service interface for REST calls in order to execute the logic of
 * component {@link Accesscodemanagement}.
 */
@Path("/accesscodemanagement/v1")
@Consumes(MediaType.APPLICATION_JSON)
@Produces(MediaType.APPLICATION_JSON)
public interface AccesscodemanagementRestService {

  /**
   * Delegates to {@link Accesscodemanagement#findAccessCodeCto}.
   *
   * @param id the ID of the {@link AccessCodeCto}
   * @return the {@link AccessCodeCto}
   */
  @GET
  @Path("/accesscode/cto/{id}/")
  public AccessCodeCto getAccessCodeCto(@PathParam("id") long id);

  /**
   * Delegates to {@link Accesscodemanagement#findAccessCodeCtos}.
   *
   * @param searchCriteriaTo the pagination and search criteria to be used for
   *                         finding accesscodes.
   * @return the {@link Page list} of matching {@link AccessCodeCto}s.
   */
  @Path("/accesscode/cto/search")
  @POST
  public Page<AccessCodeCto> findAccessCodeCtos(AccessCodeSearchCriteriaTo searchCriteriaTo);

  /**
   * Delegates to {@link Accesscodemanagement#findAccessCodeEtos}.
   *
   * @param searchCriteriaTo the pagination and search criteria to be used for
   *                         finding accesscodes.
   * @return the {@link Page list} of matching {@link AccessCodeEto}s.
   */
  @POST
  @Path("/accesscode/search")
  public Page<AccessCodeEto> findAccessCodeEtos(AccessCodeSearchCriteriaTo searchCriteriaTo);

  /**
   * Delegates to {@link Accesscodemanagement#saveAccessCode}.
   *
   * @param accessCodeEto queue the {@link AccessCodeEto} to be saved.
   * @return the recently created {@link AccessCodeEto}.
   */
  @POST
  @Path("/accesscode/")
  public AccessCodeEto saveAccessCode(AccessCodeEto accessCodeEto);

  /**
   * Delegates to {@link Accesscodemanagement#deleteAccessCode}.
   *
   * @param id of the {@link AccessCodeEto} to be deleted.
   * @return id
   */
  @DELETE
  @Path("/accesscode/{id}/")
  public long deleteAccessCode(@PathParam("id") long id);

}