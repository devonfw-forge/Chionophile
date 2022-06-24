package com.devonfw.application.api.controller;

import javax.inject.Inject;
import javax.inject.Named;

import org.springframework.data.domain.Page;

import com.devonfw.application.api.model.AccessCodeCto;
import com.devonfw.application.api.model.AccessCodeEto;
import com.devonfw.application.api.mapper.JTQMapper;
import com.devonfw.application.domain.models.AccessCodeEntity;
import com.devonfw.application.domain.repositories.AccessCodeRepository;
import com.devonfw.application.domain.tos.AccessCodeSearchCriteriaTo;


/**
 * The service implementation for REST calls in order to execute the logic of
 * component {@link Accesscodemanagement}.
 */
@Named("AccesscodemanagementRestService")
public class AccesscodemanagementRestServiceImpl implements AccesscodemanagementRestService {

  @Inject
  JTQMapper mapper;

  @Inject
  private AccessCodeRepository accessCodeRepository;

  @Override
  public AccessCodeCto getAccessCodeCto(long id) {
    AccessCodeEntity result = this.accessCodeRepository.findById(id).orElseThrow(() -> new IllegalArgumentException(
      "Entity with ID '" + id + "' was not found!"));

    return mapper.mapCto(result);
  }

  // @Override
  // public Page<AccessCodeCto> findAccessCodeCtos(AccessCodeSearchCriteriaTo searchCriteriaTo) {

  //   return this.accessCodeRepository.findByCriteria(searchCriteriaTo);
  // }

  @Override
  public AccessCodeEto saveAccessCode(AccessCodeEto accessCodeEto) {

    return mapper.map(this.accessCodeRepository.save(mapper.map(accessCodeEto)));
  }

  @Override
  public long deleteAccessCode(long id) {

    this.accessCodeRepository.deleteById(id);
    return id;
  }

   @Override
   public Page<AccessCodeEto> findAccessCodeEtos(AccessCodeSearchCriteriaTo searchCriteriaTo) {

     return this.accessCodeRepository.findByCriteria(searchCriteriaTo).map(mapper::map);
   }
}