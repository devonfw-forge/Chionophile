package com.devonfw.application.api.mapper;

import java.util.List;

import com.devonfw.application.api.model.AccessCodeCto;
import com.devonfw.application.api.model.AccessCodeEto;
import com.devonfw.application.api.model.QueueEto;
import com.devonfw.application.api.model.VisitorEto;
import org.mapstruct.InjectionStrategy;
import org.mapstruct.Mapper;
import org.mapstruct.Mapping;

import com.devonfw.application.domain.models.AccessCodeEntity;
import com.devonfw.application.domain.models.QueueEntity;
import com.devonfw.application.domain.models.VisitorEntity;
import com.devonfw.application.domain.repositories.QueueRepository;
import com.devonfw.application.domain.repositories.VisitorRepository;

@Mapper(componentModel = "cdi", injectionStrategy = InjectionStrategy.CONSTRUCTOR)
public interface JTQMapper {

  @Mapping(target = "visitor", ignore = true)
  @Mapping(target = "queue", ignore = true)
  AccessCodeEntity map(AccessCodeEto eto);

  // @Mapping(source = "visitor.id", target = "visitorId")
  // @Mapping(source = "queue.id", target = "queueId")
  AccessCodeEto map(AccessCodeEntity entity);

  @Mapping(source = ".", target = "accessCode")
  AccessCodeCto mapCto(AccessCodeEntity entity);

  // @Mapping(source = "visitor.id", target = "visitorId")
  // @Mapping(source = "queue.id", target = "queueId")
  List<AccessCodeEto> mapAccessCodeEntityList(List<AccessCodeEntity> entityList);

  VisitorEntity map(VisitorEto eto);

  VisitorEto map(VisitorEntity entity);

  // VisitorPage map(Page<VisitorEntity> entity);

  List<VisitorEto> mapVisitorEntityList(List<VisitorEntity> entityList);

  QueueEntity map(QueueEto eto);

  QueueEto map(QueueEntity entity);

  List<QueueEto> mapQueueEntityList(List<QueueEntity> entityList);
}
