package com.devonfw.application.jtqj.visitormanagement.dataaccess.api;

import javax.persistence.Entity;
import javax.persistence.Table;
import javax.validation.constraints.NotNull;

import com.devonfw.application.jtqj.general.common.api.validation.EmailExtended;
import com.devonfw.application.jtqj.general.common.api.validation.Phone;
import com.devonfw.application.jtqj.general.dataaccess.api.ApplicationPersistenceEntity;
import com.devonfw.application.jtqj.visitormanagement.common.api.Visitor;

/**
 * The type Visitor entity.
 */
@Entity
@Table(name = "Visitor")
public class VisitorEntity extends ApplicationPersistenceEntity implements Visitor {

  @NotNull
  @EmailExtended
  private String username;

  @NotNull
  private String name;

  @NotNull
  @Phone
  private String phoneNumber;

  private String password;

  private Boolean acceptedCommercial;

  private Boolean acceptedTerms;

  private Boolean userType;

  private static final long serialVersionUID = 1L;

  /**
   * Gets username.
   *
   * @return the username
   */
  public String getUsername() {

    return username;
  }

  /**
   * Sets username.
   *
   * @param username the username
   */
  public void setUsername(String username) {

    this.username = username;
  }

  /**
   * Gets name.
   *
   * @return the name
   */
  public String getName() {

    return name;
  }

  /**
   * Sets name.
   *
   * @param name the name
   */
  public void setName(String name) {

    this.name = name;
  }

  /**
   * Gets phone number.
   *
   * @return the phone number
   */
  public String getPhoneNumber() {

    return phoneNumber;
  }

  /**
   * Sets phone number.
   *
   * @param phoneNumber the phone number
   */
  public void setPhoneNumber(String phoneNumber) {

    this.phoneNumber = phoneNumber;
  }

  /**
   * Gets password.
   *
   * @return the password
   */
  public String getPassword() {

    return password;
  }

  /**
   * Sets password.
   *
   * @param password the password
   */
  public void setPassword(String password) {

    this.password = password;
  }

  /**
   * Gets accepted commercial.
   *
   * @return the accepted commercial
   */
  public Boolean getAcceptedCommercial() {

    return acceptedCommercial;
  }

  /**
   * Sets accepted commercial.
   *
   * @param acceptedCommercial the accepted commercial
   */
  public void setAcceptedCommercial(Boolean acceptedCommercial) {

    this.acceptedCommercial = acceptedCommercial;
  }

  /**
   * Gets accepted terms.
   *
   * @return the accepted terms
   */
  public Boolean getAcceptedTerms() {

    return acceptedTerms;
  }

  /**
   * Sets accepted terms.
   *
   * @param acceptedTerms the accepted terms
   */
  public void setAcceptedTerms(Boolean acceptedTerms) {

    this.acceptedTerms = acceptedTerms;
  }

  /**
   * Gets user type.
   *
   * @return the user type
   */
  public Boolean getUserType() {

    return userType;
  }

  /**
   * Sets user type.
   *
   * @param userType the user type
   */
  public void setUserType(Boolean userType) {

    this.userType = userType;
  }

}
