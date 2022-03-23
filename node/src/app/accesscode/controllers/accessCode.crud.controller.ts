import { Controller, Param } from '@nestjs/common';
import { Crud, Override, ParsedBody } from '@nestjsx/crud';
import { CrudType } from '@devon4node/common/serializer';
import { AccessCode } from '../model/entities/accessCode.entity';
import { AccessCodeCrudService } from '../services/accessCode.crud.service';
import { ApiTags } from '@nestjs/swagger';
import { AccessCodeDTO } from '../dto/accessCode';

@Crud({
  model: {
    type: AccessCode,
  },
})
@CrudType(AccessCode)
@Controller('/accesscodemanagement/v1/accesscode')
@ApiTags('AccessCode')
export class AccessCodeCrudController {
  constructor(public service: AccessCodeCrudService) {}

  @Override()
  createOne(@ParsedBody() dto: AccessCodeDTO) {
    return this.service.createOneMod(dto);
  }

  @Override()
  deleteOne(@Param('id') id: number) {
    this.service.deleteOneMod(id);
  }
}
