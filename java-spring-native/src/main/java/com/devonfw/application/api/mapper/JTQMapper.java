package com.devonfw.application.api.mapper;

import java.util.List;

import javax.inject.Inject;

import org.mapstruct.Mapper;
import org.mapstruct.Mapping;

import com.devonfw.application.api.model.AccessCodeCto;
import com.devonfw.application.api.model.AccessCodeEto;
import com.devonfw.application.api.model.QueueEto;
import com.devonfw.application.api.model.VisitorEto;
import com.devonfw.application.domain.models.AccessCodeEntity;
import com.devonfw.application.domain.models.QueueEntity;
import com.devonfw.application.domain.models.VisitorEntity;
import com.devonfw.application.domain.repositories.QueueRepository;
import com.devonfw.application.domain.repositories.VisitorRepository;


@Mapper(componentModel = "spring")
public abstract class JTQMapper {

  @Inject
  QueueRepository queueRepository;

  @Inject
  VisitorRepository visitorRepository;

  @Mapping(target = "visitor", expression = "java(visitorRepository.findById(eto.getVisitorId()).orElse(null))")
  @Mapping(target = "queue", expression = "java(queueRepository.findById(eto.getQueueId()).orElse(null))")
  public abstract AccessCodeEntity map(AccessCodeEto eto);

  @Mapping(source = "visitor.id", target = "visitorId")
  @Mapping(source = "queue.id", target = "queueId")
  public abstract AccessCodeEto map(AccessCodeEntity entity);


  public abstract AccessCodeCto mapCto(AccessCodeEntity entity);


  @Mapping(source = "visitor.id", target = "visitorId")
  @Mapping(source = "queue.id", target = "queueId")
  public abstract List<AccessCodeEto> mapAccessCodeEntityList(List<AccessCodeEntity> entityList);


  public abstract VisitorEntity map(VisitorEto eto);

  public abstract VisitorEto map(VisitorEntity entity);

  // VisitorPage map(Page<VisitorEntity> entity);

  public abstract List<VisitorEto> mapVisitorEntityList(List<VisitorEntity> entityList);

  public abstract QueueEntity map(QueueEto eto);

  public abstract QueueEto map(QueueEntity entity);

  public abstract List<QueueEto> mapQueueEntityList(List<QueueEntity> entityList);

}
