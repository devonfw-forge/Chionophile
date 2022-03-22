import { Entity, Column, PrimaryGeneratedColumn } from 'typeorm';
import { MaxLength, IsEmail } from 'class-validator';
import { Transform } from 'class-transformer';

@Entity({ name: 'visitor' })
export class Visitor {
  @PrimaryGeneratedColumn('increment')
  @Transform(id => parseInt(id))
  id: number;

  @Column('int', { name: 'modificationcounter' })
  modificationCounter?: number = 1;

  @IsEmail()
  @Column('varchar', { length: 255, nullable: true })
  username: string;

  @Column('varchar', { length: 255, nullable: true })
  name: string;

  @Column('varchar', { length: 255, nullable: true })
  password: string;

  @MaxLength(255)
  @Column('varchar', { name: 'phonenumber', length: 255, nullable: true })
  phoneNumber: string;

  @Column('bool', { name: 'acceptedcommercial', nullable: true })
  acceptedCommercial: boolean;

  @Column('bool', { name: 'acceptedterms', nullable: true })
  acceptedTerms: boolean;

  @Column('bool', { name: 'usertype', nullable: true })
  userType: boolean;
}
