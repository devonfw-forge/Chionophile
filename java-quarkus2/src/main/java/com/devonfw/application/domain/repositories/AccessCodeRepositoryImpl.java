package com.devonfw.application.domain.repositories;

import java.sql.Timestamp;
import java.util.Iterator;

import javax.inject.Inject;
import javax.persistence.EntityManager;

import org.springframework.data.domain.Page;
import org.springframework.data.domain.PageRequest;
import org.springframework.data.domain.Sort;
import org.springframework.data.domain.Sort.Order;

import com.devonfw.application.domain.models.AccessCodeEntity;
import com.devonfw.application.domain.models.QAccessCodeEntity;
import com.devonfw.application.domain.models.QQueueEntity;
import com.devonfw.application.domain.models.QVisitorEntity;
import com.devonfw.application.domain.tos.AccessCodeSearchCriteriaTo;
import com.devonfw.application.domain.utils.QueryUtil;
import com.querydsl.jpa.impl.JPAQuery;

public class AccessCodeRepositoryImpl implements AccessCodeRepositoryFragment {

  @Inject
  EntityManager em;

  public Page<AccessCodeEntity> findByCriteria(AccessCodeSearchCriteriaTo criteria) {

    QAccessCodeEntity alias = QAccessCodeEntity.accessCodeEntity;
    QVisitorEntity visitorJoin = new QVisitorEntity("visitor");
    QQueueEntity queueJoin = new QQueueEntity("queue");
    JPAQuery<AccessCodeEntity> query = new JPAQuery<AccessCodeEntity>(em);
    //important! to avoid n+1 issue, we want to return accesscode  + visitor + queue = so we need to join those tables
    //as we only want a single query to be executed(or 2 because of the count for pagination...
    query.from(QAccessCodeEntity.accessCodeEntity)
      .innerJoin(alias.visitor, visitorJoin)
      .innerJoin(alias.queue, queueJoin);
      
    Timestamp creationTime = criteria.getCreationTime();
    if (creationTime != null) {
      query.where(alias.creationTime.eq(creationTime));
    }
    Timestamp startTime = criteria.getStartTime();
    if (startTime != null) {
      query.where(alias.startTime.eq(startTime));
    }
    Timestamp endTime = criteria.getEndTime();
    if (endTime != null) {
      query.where(alias.endTime.eq(endTime));
    }
    Long visitor = criteria.getVisitorId();
    if (visitor != null) {
      query.where(alias.visitor.id.eq(visitor));
    }
    Long queue = criteria.getQueueId();
    if (queue != null) {
      query.where(alias.queue.id.eq(queue));
    }
    if (criteria.getPageable() == null) {
      criteria.setPageable(PageRequest.of(0, Integer.MAX_VALUE));
    } else {
      addOrderBy(query, alias, criteria.getPageable().getSort());
    }

    return QueryUtil.get().findPaginatedGeneric(criteria.getPageable(), query, true);
  }

  /**
   * Add sorting to the given query on the given alias
   *
   * @param query to add sorting to
   * @param alias to retrieve columns from for sorting
   * @param sort  specification of sorting
   */
  public void addOrderBy(JPAQuery<AccessCodeEntity> query, QAccessCodeEntity alias, Sort sort) {

    if (sort != null && sort.isSorted()) {
      Iterator<Order> it = sort.iterator();
      while (it.hasNext()) {
        Order next = it.next();
        switch (next.getProperty()) {
          case "creationTime":
            if (next.isAscending()) {
              query.orderBy(alias.creationTime.asc());
            } else {
              query.orderBy(alias.creationTime.desc());
            }
            break;
          case "startTime":
            if (next.isAscending()) {
              query.orderBy(alias.startTime.asc());
            } else {
              query.orderBy(alias.startTime.desc());
            }
            break;
          case "endTime":
            if (next.isAscending()) {
              query.orderBy(alias.endTime.asc());
            } else {
              query.orderBy(alias.endTime.desc());
            }
            break;
          case "visitor":
            if (next.isAscending()) {
              query.orderBy(alias.visitor.id.asc());
            } else {
              query.orderBy(alias.visitor.id.desc());
            }
            break;
          case "queue":
            if (next.isAscending()) {
              query.orderBy(alias.queue.id.asc());
            } else {
              query.orderBy(alias.queue.id.desc());
            }
            break;
          default:
            throw new IllegalArgumentException("Sorted by the unknown property '" + next.getProperty() + "'");
        }
      }
    }
  }
}
