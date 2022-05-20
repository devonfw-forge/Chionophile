package com.devonfw.application.general.common.api.mapping;

import java.util.List;

import com.devonfw.application.accesscodemanagement.dataaccess.api.AccessCodeEntity;
import com.devonfw.application.queuemanagement.dataaccess.api.QueueEntity;
import com.devonfw.application.visitormanagement.dataaccess.api.VisitorEntity;
import com.devonfw.application.accesscodemanagement.logic.api.to.AccessCodeEto;
import com.devonfw.application.queuemanagement.logic.api.to.QueueEto;
import com.devonfw.application.visitormanagement.logic.api.to.VisitorEto;
import com.devonfw.application.queuemanagement.dataaccess.api.repo.QueueRepository;
import com.devonfw.application.visitormanagement.dataaccess.api.repo.VisitorRepository;

import org.mapstruct.Mapper;
import org.mapstruct.Mapping;

@Mapper(componentModel = "cdi", uses = { QueueRepository.class, VisitorRepository.class })
public interface JTQMapper {

  @Mapping(target = "visitor", expression = "java(visitorRepository.find(eto.getVisitorId()))")
  @Mapping(target = "queue", expression = "java(queueRepository.find(eto.getQueueId()))")
  AccessCodeEntity map(AccessCodeEto eto);

  @Mapping(source = "visitor.id", target = "visitorId")
  @Mapping(source = "queue.id", target = "queueId")
  AccessCodeEto map(AccessCodeEntity entity);

  @Mapping(source = "visitor.id", target = "visitorId")
  @Mapping(source = "queue.id", target = "queueId")
  List<AccessCodeEto> mapAccessCodeEntityList(List<AccessCodeEntity> entityList);

  VisitorEntity map(VisitorEto eto);

  VisitorEto map(VisitorEntity entity);

  List<VisitorEto> mapVisitorEntityList(List<VisitorEntity> entityList);

  QueueEntity map(QueueEto eto);

  QueueEto map(QueueEntity entity);

  List<QueueEto> mapQueueEntityList(List<QueueEntity> entityList);
}
