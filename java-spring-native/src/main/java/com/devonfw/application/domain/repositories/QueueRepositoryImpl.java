package com.devonfw.application.domain.repositories;

import java.sql.Timestamp;
import java.util.Iterator;

import javax.inject.Inject;
import javax.persistence.EntityManager;

import org.springframework.data.domain.Page;
import org.springframework.data.domain.PageRequest;
import org.springframework.data.domain.Sort;
import org.springframework.data.domain.Sort.Order;

import com.devonfw.application.domain.models.QQueueEntity;
import com.devonfw.application.domain.models.QueueEntity;
import com.devonfw.application.domain.tos.QueueSearchCriteriaTo;
import com.devonfw.application.domain.utils.QueryUtil;
import com.querydsl.jpa.impl.JPAQuery;

public class QueueRepositoryImpl implements QueueRepositoryFragment {
  
  @Inject
  EntityManager em;
  
  public Page<QueueEntity> findByCriteria(QueueSearchCriteriaTo criteria) {

    QQueueEntity alias = QQueueEntity.queueEntity;
    JPAQuery<QueueEntity> query = new JPAQuery<QueueEntity>(em);
    query.from(QQueueEntity.queueEntity);
    String name = criteria.getName();
    if (name != null && !name.isEmpty()) {
      QueryUtil.get().whereString(query, alias.name, name, criteria.getNameOption());
    }
    String logo = criteria.getLogo();
    if (logo != null && !logo.isEmpty()) {
      QueryUtil.get().whereString(query, alias.logo, logo, criteria.getLogoOption());
    }
    String currentNumber = criteria.getCurrentNumber();
    if (currentNumber != null && !currentNumber.isEmpty()) {
      QueryUtil.get().whereString(query, alias.currentNumber, currentNumber, criteria.getCurrentNumberOption());
    }
    Timestamp attentionTime = criteria.getAttentionTime();
    if (attentionTime != null) {
      query.where(alias.attentionTime.eq(attentionTime));
    }
    Timestamp minAttentionTime = criteria.getMinAttentionTime();
    if (minAttentionTime != null) {
      query.where(alias.minAttentionTime.eq(minAttentionTime));
    }
    Boolean active = criteria.getActive();
    if (active != null) {
      query.where(alias.active.eq(active));
    }
    if (criteria.getPageable() == null) {
      criteria.setPageable(PageRequest.of(0, Integer.MAX_VALUE));
    } else {
      addOrderBy(query, alias, criteria.getPageable().getSort());
    }

    return QueryUtil.get().findPaginatedGeneric(criteria.getPageable(), query, true);
  }

  
  public void addOrderBy(JPAQuery<QueueEntity> query, QQueueEntity alias, Sort sort) {

    if (sort != null && sort.isSorted()) {
      Iterator<Order> it = sort.iterator();
      while (it.hasNext()) {
        Order next = it.next();
        switch (next.getProperty()) {
          case "name":
            if (next.isAscending()) {
              query.orderBy(alias.name.asc());
            } else {
              query.orderBy(alias.name.desc());
            }
            break;
          case "logo":
            if (next.isAscending()) {
              query.orderBy(alias.logo.asc());
            } else {
              query.orderBy(alias.logo.desc());
            }
            break;
          case "currentNumber":
            if (next.isAscending()) {
              query.orderBy(alias.currentNumber.asc());
            } else {
              query.orderBy(alias.currentNumber.desc());
            }
            break;
          case "attentionTime":
            if (next.isAscending()) {
              query.orderBy(alias.attentionTime.asc());
            } else {
              query.orderBy(alias.attentionTime.desc());
            }
            break;
          case "minAttentionTime":
            if (next.isAscending()) {
              query.orderBy(alias.minAttentionTime.asc());
            } else {
              query.orderBy(alias.minAttentionTime.desc());
            }
            break;
          case "active":
            if (next.isAscending()) {
              query.orderBy(alias.active.asc());
            } else {
              query.orderBy(alias.active.desc());
            }
            break;
          default:
            throw new IllegalArgumentException("Sorted by the unknown property '" + next.getProperty() + "'");
        }
      }
    }
  }
}
