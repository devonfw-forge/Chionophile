import { Controller, HttpStatus, Param, Res } from '@nestjs/common';
import { Crud, CrudRequest, Override, ParsedBody, ParsedRequest } from '@nestjsx/crud';
import { CrudType } from '@devon4node/common/serializer';
import { AccessCode } from '../model/entities/accessCode.entity';
import { AccessCodeCrudService } from '../services/accessCode.crud.service';
import { ApiTags } from '@nestjs/swagger';
import { AccessCodeDTO } from '../dto/accessCode';
import { Response } from 'express';

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

  @Override('createOneBase')
  async create(@ParsedBody() dto: AccessCodeDTO, @Res() res: Response) {
    const resData = await this.service.createOneMod(dto);
    res.status(HttpStatus.OK).send(resData);
  }

  @Override('getOneBase')
  async get(@Param('id') id: number, @Res() res: Response) {
    try {
      const resData = await this.service.getOneMod(id);
      if (resData.accessCode == undefined) {
        throw 'Inexistent AccessCode';
      }
      res.status(HttpStatus.OK).send(resData);
    } catch {
      res.status(HttpStatus.NOT_FOUND).send();
    }
  }

  @Override('deleteOneBase')
  async delete(@ParsedRequest() req: CrudRequest, @Param('id') id: number, @Res() res: Response) {
    try {
      await this.service.deleteOne(req);
      res.status(HttpStatus.OK).json(id);
    } catch {
      res.status(HttpStatus.NOT_FOUND).send();
    }
  }
}
