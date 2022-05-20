package com.devonfw.application.visitormanagement.logic.impl;

import javax.inject.Inject;

import com.devonfw.application.visitormanagement.logic.api.to.VisitorSearchCriteriaTo;
import org.junit.jupiter.api.Test;
import org.springframework.boot.test.context.SpringBootTest;

import com.devonfw.application.SpringBootApp;
import com.devonfw.application.visitormanagement.logic.api.Visitormanagement;
import com.devonfw.application.visitormanagement.logic.api.to.VisitorEto;
import com.devonfw.module.test.common.base.ComponentTest;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.PageRequest;
import org.springframework.data.domain.Pageable;

@SpringBootTest(classes = SpringBootApp.class)
public class VisitormanagementTest extends ComponentTest {

    private VisitorEto visitorEto = new VisitorEto();

    @Inject
    private Visitormanagement visitormanagement;

    @Override
    protected void doSetUp() {
        visitorEto.setName("Mary");
        visitorEto.setUsername("mary@mary.com");
        visitorEto.setPhoneNumber("123456789");
        visitorEto.setPassword("test");
        visitorEto.setUserType(false);
        visitorEto.setAcceptedTerms(true);
        visitorEto.setAcceptedCommercial(true);
    }

    @Test
    public void saveVisitorTest() {

        VisitorEto visitorEtoResult = this.visitormanagement.saveVisitor(visitorEto);

        assertThat(visitorEtoResult.getId()).isNotNull();
        assertThat(visitorEtoResult.getName()).isEqualTo("Mary");

        this.visitormanagement.deleteVisitor(visitorEtoResult.getId());
    }

    @Test
    public void findVisitorsTest() {

        VisitorSearchCriteriaTo criteria = new VisitorSearchCriteriaTo();
        Pageable pageable = PageRequest.of(0, 100);
        criteria.setPageable(pageable);
        Page<VisitorEto> result = this.visitormanagement.findVisitors(criteria);

        assertThat(result).isNotNull();
    }

}