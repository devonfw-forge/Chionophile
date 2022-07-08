package com.devonfw.application.domain.repositories;

import org.springframework.data.jpa.repository.JpaRepository;

import com.devonfw.application.domain.models.VisitorEntity;

/**
 * {@link DefaultRepository} for {@link VisitorEntity}
 */
public interface VisitorRepository extends JpaRepository<VisitorEntity, Long>, VisitorRepositoryFragment {

}