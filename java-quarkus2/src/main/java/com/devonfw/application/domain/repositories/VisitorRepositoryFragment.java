package com.devonfw.application.domain.repositories;

import org.springframework.data.domain.Page;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;

import com.devonfw.application.domain.models.VisitorEntity;
import com.devonfw.application.domain.tos.VisitorSearchCriteriaTo;

/**
 * {@link DefaultRepository} for {@link VisitorEntity}
 */
public interface VisitorRepositoryFragment {

  /**
   * @param criteria the {@link VisitorSearchCriteriaTo} with the criteria to
   *                 search.
   * @return the {@link Page} of the {@link VisitorEntity} objects that matched
   *         the search. If no pageable is set, it
   *         will return a unique page with all the objects that matched the
   *         search.
   */
  public Page<VisitorEntity> findByCriteria(VisitorSearchCriteriaTo criteria);

  /**
   * Add sorting to the given query on the given alias
   * 
   * @param query to add sorting to
   * @param alias to retrieve columns from for sorting
   * @param sort  specification of sorting
   */
  // public void addOrderBy(JPAQuery<VisitorEntity> query, VisitorEntity alias,
  // Sort sort);

}