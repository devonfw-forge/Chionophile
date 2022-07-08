package com.devonfw.application.api.controller;

import javax.inject.Inject;
import javax.inject.Named;

import org.springframework.data.domain.Page;

import com.devonfw.application.api.mapper.JTQMapper;
import com.devonfw.application.api.model.VisitorEto;
import com.devonfw.application.domain.models.VisitorEntity;
import com.devonfw.application.domain.repositories.VisitorRepository;
import com.devonfw.application.domain.tos.VisitorSearchCriteriaTo;

/**
 * The service implementation for REST calls in order to execute the logic of
 * component {@link Visitormanagement}.
 */
@Named("VisitormanagementRestService")
public class VisitormanagementRestServiceImpl implements VisitormanagementRestService {

  @Inject
  JTQMapper mapper;

  @Inject
  VisitorRepository visitorrepo;

  @Override
  public VisitorEto getVisitor(long id) {
    VisitorEntity result = this.visitorrepo.findById(id).orElseThrow(() -> new IllegalArgumentException(
        "Entity with ID '" + id + "' was not found!"));

    return mapper.map(result);
  }

  @Override
  public VisitorEto saveVisitor(VisitorEto visitor) {

    return mapper.map(this.visitorrepo.save(mapper.map(visitor)));
  }

  @Override
  public long deleteVisitor(long id) {

    this.visitorrepo.deleteById(id);
    return id;
  }

  @Override
  public Page<VisitorEto> findVisitors(VisitorSearchCriteriaTo searchCriteriaTo) {
    return this.visitorrepo.findByCriteria(searchCriteriaTo).map(mapper::map);
  }
}