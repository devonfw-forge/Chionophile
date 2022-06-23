package com.devonfw.application.api.model;

import java.util.List;

import org.mapstruct.Mapper;
import org.mapstruct.Mapping;
import org.springframework.data.domain.Page;

import com.devonfw.application.domain.models.VisitorEntity;

@Mapper(componentModel = "cdi" /* uses = { QueueRepository.class, VisitorRepository.class } */)
public interface JTQMapper {

  // @Mapping(target = "visitor", expression =
  // "java(visitorRepository.find(eto.getVisitorId()))")
  // @Mapping(target = "queue", expression =
  // "java(queueRepository.find(eto.getQueueId()))")
  // AccessCodeEntity map(AccessCodeEto eto);

  // @Mapping(source = "visitor.id", target = "visitorId")
  // @Mapping(source = "queue.id", target = "queueId")
  // AccessCodeEto map(AccessCodeEntity entity);

  // @Mapping(source = "visitor.id", target = "visitorId")
  // @Mapping(source = "queue.id", target = "queueId")
  // List<AccessCodeEto> mapAccessCodeEntityList(List<AccessCodeEntity>
  // entityList);

  VisitorEntity map(VisitorEto eto);

  VisitorEto map(VisitorEntity entity);

  VisitorPage map(Page<VisitorEntity> entity);

  List<VisitorEto> mapVisitorEntityList(List<VisitorEntity> entityList);

  // QueueEntity map(QueueEto eto);

  // QueueEto map(QueueEntity entity);

  // List<QueueEto> mapQueueEntityList(List<QueueEntity> entityList);
}
