package com.devonfw.application.domain.repositories;

import org.springframework.data.domain.Page;
import org.springframework.data.jpa.repository.JpaRepository;

import com.devonfw.application.domain.models.QueueEntity;
import com.devonfw.application.domain.tos.QueueSearchCriteriaTo;

/**
 * {@link DefaultRepository} for {@link QueueEntity}
 */
public interface QueueRepository extends JpaRepository<QueueEntity, Long>, QueueRepositoryFragment {

  /**
   * @param criteria the {@link QueueSearchCriteriaTo} with the criteria to
   *                 search.
   * @return the {@link Page} of the {@link QueueEntity} objects that matched the
   *         search. If no pageable is set, it will
   *         return a unique page with all the objects that matched the search.
   */
  public Page<QueueEntity> findByCriteria(QueueSearchCriteriaTo criteria);

  /**
   * Add sorting to the given query on the given alias
   *
   * @param query to add sorting to
   * @param alias to retrieve columns from for sorting
   * @param sort  specification of sorting
   */
  // public default void addOrderBy(JPAQuery<QueueEntity> query, QueueEntity alias, Sort sort);

}