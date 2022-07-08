package com.devonfw.application.api.controller;

import javax.enterprise.context.RequestScoped;
import javax.inject.Inject;

import org.springframework.data.domain.Page;

import com.devonfw.application.api.mapper.JTQMapper;
import com.devonfw.application.api.model.QueueEto;
import com.devonfw.application.domain.models.QueueEntity;
import com.devonfw.application.domain.repositories.QueueRepository;
import com.devonfw.application.domain.tos.QueueSearchCriteriaTo;

/**
 * The service implementation for REST calls in order to execute the logic of
 * component {@link Queuemanagement}.
 */
@RequestScoped
public class QueuemanagementRestServiceImpl implements QueuemanagementRestService {

  @Inject
  JTQMapper mapper;

  @Inject
  QueueRepository repository;

  @Override
  public QueueEto getQueue(long id) {
    QueueEntity result = this.repository.findById(id).orElseThrow(() -> new IllegalArgumentException(
        "Entity with ID '" + id + "' was not found!"));
    return mapper.map(result);
  }

  @Override
  public QueueEto saveQueue(QueueEto queue) {

    return mapper.map(this.repository.save(mapper.map(queue)));
  }

  @Override
  public long deleteQueue(long id) {

    this.repository.deleteById(id);
    return id;
  }

   @Override
   public Page<QueueEto> findQueues(QueueSearchCriteriaTo searchCriteriaTo) {
     return this.repository.findByCriteria(searchCriteriaTo).map(mapper::map);
   }
}
