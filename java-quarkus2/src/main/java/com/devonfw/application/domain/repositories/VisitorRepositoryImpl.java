package com.devonfw.application.domain.repositories;

import java.util.Iterator;

import javax.inject.Inject;
import javax.persistence.EntityManager;

import org.springframework.data.domain.Page;
import org.springframework.data.domain.PageRequest;
import org.springframework.data.domain.Sort;

import com.devonfw.application.domain.models.QVisitorEntity;
import com.devonfw.application.domain.models.VisitorEntity;
import com.devonfw.application.domain.tos.VisitorSearchCriteriaTo;
import com.devonfw.application.domain.utils.QueryUtil;
import com.querydsl.jpa.impl.JPAQuery;
import org.springframework.data.domain.Sort.Order;

public class VisitorRepositoryImpl implements VisitorRepositoryFragment {

  @Inject
  EntityManager em;

  public Page<VisitorEntity> findByCriteria(VisitorSearchCriteriaTo criteria) {

    QVisitorEntity alias = QVisitorEntity.visitorEntity;
    JPAQuery<VisitorEntity> query = new JPAQuery<VisitorEntity>(em);
    query.from(QVisitorEntity.visitorEntity);
    String username = criteria.getUsername();
    if (username != null && !username.isEmpty()) {
      QueryUtil.get().whereString(query, alias.username, username,
          criteria.getUsernameOption());
    }
    String name = criteria.getName();
    if (name != null && !name.isEmpty()) {
      QueryUtil.get().whereString(query, alias.name, name,
          criteria.getNameOption());
    }
    String phoneNumber = criteria.getPhoneNumber();
    if (phoneNumber != null && !phoneNumber.isEmpty()) {
      QueryUtil.get().whereString(query, alias.phoneNumber, phoneNumber,
          criteria.getPhoneNumberOption());
    }
    String password = criteria.getPassword();
    if (password != null && !password.isEmpty()) {
      QueryUtil.get().whereString(query, alias.password, password,
          criteria.getPasswordOption());
    }
    Boolean acceptedCommercial = criteria.getAcceptedCommercial();
    if (acceptedCommercial != null) {
      query.where(alias.acceptedCommercial.eq(acceptedCommercial));
    }
    Boolean acceptedTerms = criteria.getAcceptedTerms();
    if (acceptedTerms != null) {
      query.where(alias.acceptedTerms.eq(acceptedTerms));
    }
    Boolean userType = criteria.getUserType();
    if (userType != null) {
      query.where(alias.userType.eq(userType));
    }
    if (criteria.getPageable() == null) {
      criteria.setPageable(PageRequest.of(0, Integer.MAX_VALUE));
    } else {
      addOrderBy(query, alias, criteria.getPageable().getSort());
    }

    return QueryUtil.get().findPaginatedGeneric(criteria.getPageable(), query, true);
  }

  public void addOrderBy(JPAQuery<VisitorEntity> query, QVisitorEntity alias, Sort sort) {

    if (sort != null && sort.isSorted()) {
      Iterator<Order> it = sort.iterator();
      while (it.hasNext()) {
        Order next = it.next();
        switch (next.getProperty()) {
          case "username":
            if (next.isAscending()) {
              query.orderBy(alias.username.asc());
            } else {
              query.orderBy(alias.username.desc());
            }
            break;
          case "name":
            if (next.isAscending()) {
              query.orderBy(alias.name.asc());
            } else {
              query.orderBy(alias.name.desc());
            }
            break;
          case "phoneNumber":
            if (next.isAscending()) {
              query.orderBy(alias.phoneNumber.asc());
            } else {
              query.orderBy(alias.phoneNumber.desc());
            }
            break;
          case "password":
            if (next.isAscending()) {
              query.orderBy(alias.password.asc());
            } else {
              query.orderBy(alias.password.desc());
            }
            break;
          case "acceptedCommercial":
            if (next.isAscending()) {
              query.orderBy(alias.acceptedCommercial.asc());
            } else {
              query.orderBy(alias.acceptedCommercial.desc());
            }
            break;
          case "acceptedTerms":
            if (next.isAscending()) {
              query.orderBy(alias.acceptedTerms.asc());
            } else {
              query.orderBy(alias.acceptedTerms.desc());
            }
            break;
          case "userType":
            if (next.isAscending()) {
              query.orderBy(alias.userType.asc());
            } else {
              query.orderBy(alias.userType.desc());
            }
            break;
          default:
            throw new IllegalArgumentException("Sorted by the unknown property '" +
                next.getProperty() + "'");
        }
      }
    }
  }
}
