package com.devonfw.application.domain.repositories;

import org.springframework.data.domain.Page;
import org.springframework.data.jpa.repository.JpaRepository;

import com.devonfw.application.domain.models.VisitorEntity;
import com.devonfw.application.domain.tos.VisitorSearchCriteriaTo;

/**
 * {@link DefaultRepository} for {@link VisitorEntity}
 */
public interface VisitorRepository extends JpaRepository<VisitorEntity, Long>, VisitorRepositoryFragment {



}